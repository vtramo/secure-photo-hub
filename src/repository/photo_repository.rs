use anyhow::Context;
use chrono::Utc;
use sqlx::{Acquire, PgConnection, query_file};
use sqlx::types::uuid;
use uuid::Uuid;

use crate::models::entity::{ImageFormatEntity, PhotoEntity, VisibilityEntity};
use crate::models::entity::ImageEntity;
use crate::models::service::CreatePhoto;
use crate::repository::PostgresDatabase;

#[async_trait::async_trait]
pub trait PhotoRepository {
    async fn create(&self, photo: CreatePhoto) -> anyhow::Result<PhotoEntity>;
}

#[async_trait::async_trait]
impl PhotoRepository for PostgresDatabase {
    async fn create(&self, create_photo: CreatePhoto) -> anyhow::Result<PhotoEntity> {
        let mut conn = self
            .acquire()
            .await
            .with_context(|| "Unable to acquire a database connection".to_string())?;

        let mut tx = conn.begin().await?;

        let created_image_entity = Self::insert_image(
            create_photo.url(),
            create_photo.image_id(),
            ImageFormatEntity::from(create_photo.format()),
            create_photo.size(),
            &mut *tx,
        ).await?;

        let created_photo_entity = Self::insert_photo(
            create_photo,
            created_image_entity,
            &mut tx
        ).await?;

        tx.commit().await?;

        Ok(created_photo_entity)
    }
}

impl PostgresDatabase {
    async fn insert_image(
        url: &url::Url,
        image_id: Uuid,
        format: ImageFormatEntity,
        file_size: u64,
        conn: &mut PgConnection,
    ) -> anyhow::Result<ImageEntity> {
        let created_at = Utc::now();

        query_file!(
            "queries/postgres/insert_image.sql",
            image_id,
            url.to_string(),
            file_size as i64,
            format as _
        ).execute(conn)
        .await?;

        Ok(ImageEntity {
            id: image_id,
            url: url.clone(),
            size: file_size,
            format,
            created_at,
        })
    }

    async fn insert_photo(
        create_photo: CreatePhoto,
        image_entity: ImageEntity,
        conn: &mut PgConnection,
    ) -> anyhow::Result<PhotoEntity> {
        let photo_id = Uuid::new_v4();
        let title = create_photo.title().to_string();
        let description = create_photo.description().to_string();
        let visibility = VisibilityEntity::from(create_photo.visibility().clone());
        let owner_user_id = create_photo.owner_user_id().clone();
        let tags = create_photo.tags().clone();
        let category = create_photo.category().to_string();
        let album_id = create_photo.album_id().unwrap_or(Uuid::nil());
        let image_id = image_entity.id.clone();

        query_file!(
            "queries/postgres/insert_photo.sql",
            photo_id,
            title,
            description,
            visibility as _,
            owner_user_id,
            &tags,
            category,
            album_id,
            image_id
        ).execute(conn)
            .await?;

        Ok(PhotoEntity {
            id: photo_id,
            album_id: if album_id.is_nil() { None } else { Some(album_id) },
            owner_user_id,
            title,
            description,
            tags,
            category,
            visibility,
            image: image_entity,
            created_at: Utc::now(),
            is_deleted: false,
        })
    }
}

#[allow(unused_imports)]
mod tests {
    use image::ImageFormat;
    use url::Url;
    use uuid::Uuid;

    use crate::models::service::{CreatePhoto, Visibility};
    use crate::repository::photo_repository::PhotoRepository;
    use crate::repository::PostgresDatabase;

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test() {
        let env: &'static str = env!("DATABASE_URL");
        let pg = PostgresDatabase::connect(env).await.unwrap();

        let owner_user_id = Uuid::new_v4();
        let image_id = Uuid::new_v4();
        let image_url = Url::parse("http://localhost:8080/").unwrap();
        let create_photo = CreatePhoto::new(
            "title".to_string(),
            "description".to_string(),
            "category".to_string(),
            vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            owner_user_id,
            image_id,
            None,
            Visibility::Private,
            image_url.clone(),
            1024,
            ImageFormat::Png,
        );

        let created_photo = pg.create(create_photo).await.unwrap();

        dbg!(&created_photo);
        assert_eq!(owner_user_id, created_photo.owner_user_id);
        assert_eq!(image_url, created_photo.image.url);
        assert_eq!(image_id, created_photo.image.id);
    }
}