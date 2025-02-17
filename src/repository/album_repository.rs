use anyhow::{anyhow, Context};
use sqlx::{Acquire, PgConnection, query_file_as};
use uuid::Uuid;

use crate::models::entity::{ImageReferenceEntity, VisibilityEntity};
use crate::models::entity::album::{AlbumCoverImageReferenceEntity, AlbumEntity, AlbumNoCoverImageReferenceEntity};
use crate::models::service::album::{CreateAlbum, UpdateAlbum};
use crate::models::service::image::ImageReference;
use crate::repository::{NULL, PostgresDatabase};

#[async_trait::async_trait]
pub trait AlbumRepository: Clone + Send + Sync + 'static {
    async fn create_album(&self, album: &CreateAlbum) -> anyhow::Result<AlbumEntity>;
    async fn find_all_albums(&self) -> anyhow::Result<Vec<AlbumEntity>>;
    async fn find_album_by_id(&self, id: &Uuid) -> anyhow::Result<Option<AlbumEntity>>;
    async fn update_album(&self, update_album: &UpdateAlbum) -> anyhow::Result<AlbumEntity>;
}

#[async_trait::async_trait]
impl AlbumRepository for PostgresDatabase {
    async fn create_album(&self, create_album: &CreateAlbum) -> anyhow::Result<AlbumEntity> {
        let mut conn = self
            .acquire()
            .await
            .with_context(|| "Unable to acquire a database connection".to_string())?;

        let mut tx = conn.begin().await?;

        let cover_image_reference = ImageReference::new(
            create_album.cover_image_id(),    
            create_album.owner_user_id(),
            create_album.cover_image_url(),
            create_album.cover_image_size(),
            create_album.cover_image_format(),
            create_album.visibility(),
        );
        
        let album_cover_image = Self::insert_image_reference(
            &cover_image_reference,
            &mut *tx,
        ).await?;

        let created_album_entity = Self::insert_album(
            create_album,
            &album_cover_image,
            &mut *tx
        ).await?;

        tx.commit().await?;

        Ok(created_album_entity)
    }

    async fn find_all_albums(&self) -> anyhow::Result<Vec<AlbumEntity>> {
        let mut conn = self
            .acquire()
            .await
            .with_context(|| "Unable to acquire a database connection".to_string())?;

        let album_entities: Vec<_> = query_file_as!(AlbumCoverImageReferenceEntity, "queries/postgres/find_all_albums.sql")
            .fetch_all(&mut *conn)
            .await?;

        Ok(album_entities.into_iter().map(AlbumEntity::from).collect())
    }

    async fn find_album_by_id(&self, id: &Uuid) -> anyhow::Result<Option<AlbumEntity>> {
        let mut conn = self
            .acquire()
            .await
            .with_context(|| "Unable to acquire a database connection".to_string())?;

        let option_album: Option<AlbumCoverImageReferenceEntity> = 
            query_file_as!(AlbumCoverImageReferenceEntity, "queries/postgres/find_album_by_id.sql", id)
                .fetch_optional(&mut *conn)
                .await?;
        
        Ok(option_album.map(AlbumEntity::from))
    }

    async fn update_album(&self, update_album: &UpdateAlbum) -> anyhow::Result<AlbumEntity> {
        let mut conn = self.acquire()
            .await
            .with_context(|| "Unable to acquire a database connection".to_string())?;

        let album_id = update_album.id();
        let title = update_album.title().clone().unwrap_or(String::from(NULL));
        let visibility = update_album.visibility().clone().map(VisibilityEntity::from).unwrap_or(VisibilityEntity::Null);

        let updated_album_entity: AlbumCoverImageReferenceEntity =
            query_file_as!(
                AlbumCoverImageReferenceEntity,
                "queries/postgres/update_album.sql",
                album_id,
                title,
                visibility as _,
            ).fetch_all(&mut *conn)
                .await.map_err(|err| anyhow!("Unable to update an album {}", err))?
                .get(0)
                .cloned()
                .take()
                .ok_or(anyhow!("Unable to update an album"))?;

        Ok(AlbumEntity::from(updated_album_entity))
    }
}

impl PostgresDatabase {
    async fn insert_album(
        create_album: &CreateAlbum,
        cover_image_entity: &ImageReferenceEntity,
        conn: &mut PgConnection
    ) -> anyhow::Result<AlbumEntity> {
        let album_id = Uuid::new_v4();
        let title = create_album.title().to_string();
        let description = create_album.description().to_string();
        let owner_user_id = create_album.owner_user_id().clone();
        let cover_image_id = create_album.cover_image_id().clone();
        let visibility = VisibilityEntity::from(create_album.visibility().clone());
        
        let created_album: AlbumNoCoverImageReferenceEntity = query_file_as!(
            AlbumNoCoverImageReferenceEntity,
            "queries/postgres/insert_album.sql",
            album_id,
            title,
            description,
            visibility as _,
            owner_user_id,
            cover_image_id
        ).fetch_one(conn)
            .await?;
        
        Ok(AlbumEntity {
            id: created_album.id,
            owner_user_id,
            title,
            description,
            visibility,
            cover_image: ImageReferenceEntity {
                id: cover_image_entity.id,
                owner_user_id: cover_image_entity.owner_user_id,
                url: create_album.cover_image_reference_url().to_string(),
                size: cover_image_entity.size,
                visibility,
                format: cover_image_entity.format.clone(),
                created_at: cover_image_entity.created_at,
            },
            created_at: created_album.created_at,
        })
    }
}

#[allow(unused_imports)]
mod tests {
    use image::ImageFormat;
    use url::Url;
    use uuid::Uuid;

    use crate::models::service::photo::{CreatePhoto, UpdatePhoto};
    use crate::models::service::Visibility;
    use crate::repository::photo_repository::PhotoRepository;

    use super::*;

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn should_return_album_with_correct_details_after_creation() {
        let env: &'static str = env!("DATABASE_URL"); // TODO: fixtures
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
            cover_image_url.clone(),
            2048,
            ImageFormat::Jpeg,
        );

        let created_album = pg.create_album(&create_album).await.unwrap();

        assert_eq!(owner_user_id, created_album.owner_user_id);
        assert_eq!(title, created_album.title);
        assert_eq!(description, created_album.description);
        assert_eq!(visibility, created_album.visibility.into());
        assert_eq!(cover_image_id, created_album.cover_image.id);
        assert_eq!(cover_image_url.to_string(), created_album.cover_image.url);
    }

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn should_move_photo_to_album() {
        let env: &'static str = env!("DATABASE_URL");
        let pg = PostgresDatabase::connect(env).await.unwrap();

        let owner_user_id = Uuid::new_v4();
        let image_id = Uuid::new_v4();
        let image_url = Url::parse("http://localhost:8080/").unwrap();
        let create_photo = CreatePhoto::new(
            "title",
            "description",
            "category",
            &vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            &owner_user_id,
            &image_id,
            &None,
            &Visibility::Private,
            &image_url,
            &image_url,
            1024,
            &ImageFormat::Png,
        );
        let created_photo = pg.create_photo(&create_photo).await.expect("");
        assert_eq!(created_photo.album_id, Some(Uuid::nil()));

        let title = "New Album".to_string();
        let description = "Album description".to_string();
        let visibility = Visibility::Private;
        let cover_image_id = Uuid::new_v4();
        let cover_image_url = Url::parse("http://localhost:8080/cover_image").unwrap();
        let create_album = CreateAlbum::new(
            title.clone(),
            description,
            visibility,
            owner_user_id,
            cover_image_id,
            cover_image_url.clone(),
            cover_image_url.clone(),
            2048,
            ImageFormat::Jpeg,
        );
        let created_album = pg.create_album(&create_album).await.unwrap();
        assert_eq!(created_album.title, title);
        assert_eq!(created_album.owner_user_id, owner_user_id);

        let album_id = created_album.id;
        let update_photo = UpdatePhoto::new(&created_photo.id, None, Some(&album_id), None);
        let moved_photo = pg.update_photo(&update_photo).await.unwrap();
        assert_eq!(moved_photo.album_id, Some(album_id));
    }

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn should_create_and_find_album() {
        let env: &'static str = env!("DATABASE_URL");
        let pg = PostgresDatabase::connect(env).await.unwrap();

        let owner_user_id = Uuid::new_v4();
        let cover_image_id = Uuid::new_v4();
        let cover_image_url = Url::parse("http://localhost:8080/cover_image").unwrap();
        let title = "New Album".to_string();
        let description = "Album description".to_string();
        let visibility = Visibility::Private;

        let create_album = CreateAlbum::new(
            title.clone(),
            description.clone(),
            visibility.clone(),
            owner_user_id,
            cover_image_id,
            cover_image_url.clone(),
            cover_image_url.clone(),
            2048,
            ImageFormat::Jpeg,
        );

        let created_album = pg.create_album(&create_album).await.unwrap();

        assert_eq!(created_album.title, title);
        assert_eq!(created_album.description, description);
        assert_eq!(created_album.visibility, visibility.into());
        assert_eq!(created_album.owner_user_id, owner_user_id);

        let albums = pg.find_all_albums().await.unwrap();

        assert!(albums.iter().any(|album| album.id == created_album.id));
    }

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn should_find_album_by_id() {
        let env: &'static str = env!("DATABASE_URL");
        let pg = PostgresDatabase::connect(env).await.unwrap();

        let random_album_id = Uuid::new_v4();
        let found_album = pg.find_album_by_id(&random_album_id).await.unwrap();
        assert!(found_album.is_none());

        let owner_user_id = Uuid::new_v4();
        let cover_image_id = Uuid::new_v4();
        let cover_image_url = Url::parse("http://localhost:8080/cover_image").unwrap();
        let title = "New Album".to_string();
        let description = "Album description".to_string();
        let visibility = Visibility::Private;

        let create_album = CreateAlbum::new(
            title,
            description,
            visibility,
            owner_user_id,
            cover_image_id,
            cover_image_url.clone(),
            cover_image_url.clone(),
            2048, 
            ImageFormat::Jpeg,
        );

        let created_album = pg.create_album(&create_album).await.unwrap();

        let found_album = pg.find_album_by_id(&created_album.id).await.unwrap();
        assert_eq!(found_album, Some(created_album));
    }

}