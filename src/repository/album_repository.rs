use anyhow::Context;
use sqlx::{Acquire, PgConnection, query_file};
use uuid::Uuid;

use crate::models::entity::{ImageEntity, ImageFormatEntity, VisibilityEntity};
use crate::models::entity::album::AlbumEntity;
use crate::models::service::album::CreateAlbum;
use crate::repository::PostgresDatabase;

#[async_trait::async_trait]
pub trait AlbumRepository {
    async fn create_album(&self, album: CreateAlbum) -> anyhow::Result<AlbumEntity>;
}

#[async_trait::async_trait]
impl AlbumRepository for PostgresDatabase {
    async fn create_album(&self, create_album: CreateAlbum) -> anyhow::Result<AlbumEntity> {
        let mut conn = self
            .acquire()
            .await
            .with_context(|| "Unable to acquire a database connection".to_string())?;

        let mut tx = conn.begin().await?;

        let created_cover_image_entity = Self::insert_image(
            create_album.cover_image_url(),
            create_album.cover_image_id(),
            ImageFormatEntity::from(create_album.cover_image_format()),
            create_album.cover_image_size(),
            &mut *tx,
        ).await?;
        
        let created_album_entity = Self::insert_album(
            create_album,
            created_cover_image_entity,
            &mut *tx
        ).await?;
        
        tx.commit().await?;
        
        Ok(created_album_entity)
    }
}

impl PostgresDatabase {
    async fn insert_album(
        create_album: CreateAlbum,
        cover_image: ImageEntity,
        conn: &mut PgConnection
    ) -> anyhow::Result<AlbumEntity> {
        let album_id = Uuid::new_v4();
        let title = create_album.title().to_string();
        let description = create_album.description().to_string();
        let owner_user_id = create_album.owner_user_id().clone();
        let cover_image_id = create_album.cover_image_id().clone();
        let visibility = VisibilityEntity::from(create_album.visibility().clone());


        query_file!(
            "queries/postgres/insert_album.sql",
            album_id,
            title,
            description,
            visibility as _,
            owner_user_id,
            cover_image_id
        ).execute(conn)
            .await?;

        Ok(AlbumEntity {
            id: album_id,
            owner_user_id,
            title,
            description,
            visibility,
            cover_image,
            created_at: Default::default(),
        })
    }
}

#[allow(unused_imports)]
mod tests {
    use image::ImageFormat;
    use url::Url;
    use uuid::Uuid;

    use crate::models::service::album::CreateAlbum;
    use crate::models::service::Visibility;
    use crate::repository::album_repository::AlbumRepository;
    use crate::repository::PostgresDatabase;

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn should_return_album_with_correct_details_after_creation() {
        let env: &'static str = env!("DATABASE_URL");
        let pg = PostgresDatabase::connect(env).await.unwrap();

        let owner_user_id = Uuid::new_v4();
        let cover_image_id = Uuid::new_v4();
        let cover_image_url = Url::parse("http://localhost:8080/cover_image").unwrap();
        let title = "Album Title".to_string();
        let description = "Album description".to_string();
        let visibility = Visibility::Private;

        let create_album = CreateAlbum::new(
            title.clone(),
            description.clone(),
            visibility.clone(),
            owner_user_id,
            cover_image_id,
            cover_image_url.clone(),
            2048,
            ImageFormat::Jpeg,
        );

        let created_album = pg.create_album(create_album).await.unwrap();

        assert_eq!(owner_user_id, created_album.owner_user_id);
        assert_eq!(title, created_album.title);
        assert_eq!(description, created_album.description);
        assert_eq!(visibility, created_album.visibility.into());
        assert_eq!(cover_image_url, created_album.cover_image.url);
        assert_eq!(cover_image_id, created_album.cover_image.id);
    }
}