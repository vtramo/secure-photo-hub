use anyhow::{anyhow, Context};
use sqlx::{Acquire, PgConnection, query_file_as};
use sqlx::types::uuid;
use uuid::Uuid;

use crate::models::entity::{ImageFormatEntity, ImageReferenceEntity, VisibilityEntity};
use crate::models::entity::photo::{PhotoEntity, PhotoImageReferenceEntity, PhotoNoImageReferenceEntity};
use crate::models::service::photo::{CreatePhoto, UpdatePhoto};
use crate::repository::{NULL, PostgresDatabase};

#[async_trait::async_trait]
pub trait PhotoRepository: Clone + Send + Sync + 'static {
    async fn create_photo(&self, photo: &CreatePhoto) -> anyhow::Result<PhotoEntity>;
    async fn find_all_photos(&self, limit: u32, offset: u32) -> anyhow::Result<Vec<PhotoEntity>>;
    async fn find_photo_by_id(&self, id: &Uuid) -> anyhow::Result<Option<PhotoEntity>>;
    async fn update_photo(&self, photo: &UpdatePhoto) -> anyhow::Result<PhotoEntity>;
}

#[async_trait::async_trait]
impl PhotoRepository for PostgresDatabase {
    async fn create_photo(&self, photo: &CreatePhoto) -> anyhow::Result<PhotoEntity> {
        let mut conn = self
            .acquire()
            .await
            .with_context(|| "Unable to acquire a database connection".to_string())?;

        let mut tx = conn.begin().await?;

        let created_image_entity = Self::insert_image_reference(
            photo.url(),
            photo.image_id(),
            ImageFormatEntity::from(photo.format()),
            photo.size(),
            &mut *tx,
        ).await?;

        let created_photo_entity = Self::insert_photo(
            photo,
            &created_image_entity,
            &mut tx
        ).await?;

        tx.commit().await?;

        Ok(created_photo_entity)
    }

    async fn find_all_photos(&self, limit: u32, offset: u32) -> anyhow::Result<Vec<PhotoEntity>> {
        let mut conn = self.acquire()
            .await
            .with_context(|| "Unable to acquire a database connection".to_string())?;

        let photo_image_entities: Vec<_> = query_file_as!(
            PhotoImageReferenceEntity, 
            "queries/postgres/find_all_photo.sql",
            limit as i64,
            offset as i64
        )
            .fetch_all(&mut *conn)
            .await?;

        Ok(photo_image_entities.into_iter().map(PhotoEntity::from).collect())
    }

    async fn find_photo_by_id(&self, id: &Uuid) -> anyhow::Result<Option<PhotoEntity>> {
        let mut conn = self.acquire()
            .await
            .with_context(|| "Unable to acquire a database connection".to_string())?;

        let photo_image_entity: Option<PhotoImageReferenceEntity> = query_file_as!(
            PhotoImageReferenceEntity,
            "queries/postgres/find_photo_by_id.sql",
            id
        ).fetch_optional(&mut *conn)
        .await?;

        Ok(photo_image_entity.map(PhotoEntity::from))
    }

    async fn update_photo(&self, update_photo: &UpdatePhoto) -> anyhow::Result<PhotoEntity> {
        let mut conn = self.acquire()
            .await
            .with_context(|| "Unable to acquire a database connection".to_string())?;
        
        let photo_id = update_photo.id();
        let album_id = update_photo.album_id().clone().unwrap_or(Uuid::nil());
        let title = update_photo.title().clone().unwrap_or(String::from(NULL));
        
        let updated_photo_entity: PhotoImageReferenceEntity = query_file_as!(
            PhotoImageReferenceEntity,
            "queries/postgres/update_photo.sql",
            photo_id,
            album_id,
            title,
        ).fetch_all(&mut *conn)
            .await.map_err(|err| anyhow!("Unable to update a photo {}", err))?
            .get(0)
            .cloned()
            .take()
            .ok_or(anyhow!("Unable to update a photo"))?;
        
        Ok(PhotoEntity::from(updated_photo_entity))
    }
}

impl PostgresDatabase {
    pub async fn insert_image_reference(
        url: &url::Url,
        image_id: Uuid,
        format: ImageFormatEntity,
        file_size: u64,
        conn: &mut PgConnection,
    ) -> anyhow::Result<ImageReferenceEntity> {
        let created_image: ImageReferenceEntity = query_file_as!(
            ImageReferenceEntity,
            "queries/postgres/insert_image_reference.sql",
            image_id,
            url.to_string(),
            file_size as i64,
            format as _
        ).fetch_all(conn)
            .await?
            .get(0)
            .cloned()
            .take()
            .ok_or(anyhow!("Unable to insert an image"))?;

        Ok(created_image)
    }

    pub async fn insert_photo(
        create_photo: &CreatePhoto,
        image_entity: &ImageReferenceEntity,
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
        let image_id = create_photo.image_id().clone();

        let created_photo_image_entity: PhotoNoImageReferenceEntity = query_file_as!(
            PhotoNoImageReferenceEntity,
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
        ).fetch_all(conn)
            .await?
            .get(0)
            .cloned()
            .take()
            .ok_or(anyhow!("Unable to insert a photo"))?;

        Ok(PhotoEntity {
            id: created_photo_image_entity.id,
            album_id: created_photo_image_entity.album_id,
            owner_user_id,
            title,
            description,
            tags,
            category,
            visibility,
            image: image_entity.clone(),
            created_at: created_photo_image_entity.created_at,
            is_deleted: created_photo_image_entity.is_deleted,
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
    use crate::repository::PostgresDatabase;

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn should_return_photo_with_correct_details_after_creation() {
        let env: &'static str = env!("DATABASE_URL");
        let pg = PostgresDatabase::connect(env).await.unwrap();

        let owner_user_id = Uuid::new_v4();
        let image_id = Uuid::new_v4();
        let image_url = Url::parse("http://localhost:8080/").unwrap();
        let album_id = uuid::Uuid::new_v4();
        let create_photo = CreatePhoto::new(
            "title",
            "description",
            "category",
            &vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            &owner_user_id,
            &image_id,
            &Some(album_id),
            &Visibility::Private,
            &image_url,
            1024,
            &ImageFormat::Png,
        );

        let created_photo = pg.create_photo(&create_photo).await.unwrap();

        assert_eq!(owner_user_id, created_photo.owner_user_id);
        assert_eq!(image_url.to_string(), created_photo.image.url);
        assert_eq!(image_id, created_photo.image.id);
        assert_eq!(Some(album_id), created_photo.album_id);
    }

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn should_return_all_photos_when_find_all_photos() {
        let env: &'static str = env!("DATABASE_URL");
        let pg = PostgresDatabase::connect(env).await.unwrap();

        let photos = pg.find_all_photos(30, 0).await.unwrap();
        
        for photo in photos {
            assert!(photo.id.is_nil() == false, "Photo ID should be valid");
        }
    }

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn find_photo_by_id() {
        let env: &'static str = env!("DATABASE_URL");
        let pg = PostgresDatabase::connect(env).await.unwrap();

        let owner_user_id = Uuid::new_v4();
        let image_id = Uuid::new_v4();
        let image_url = Url::parse("http://localhost:8080/").unwrap();
        let album_id = uuid::Uuid::new_v4();
        let create_photo = CreatePhoto::new(
            "title",
            "description",
            "category",
            &vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            &owner_user_id,
            &image_id,
            &Some(album_id),
            &Visibility::Private,
            &image_url,
            1024,
            &ImageFormat::Png,
        );

        let created_photo = pg.create_photo(&create_photo).await.unwrap();
        assert_eq!(image_id, created_photo.image.id);

        let photo = pg.find_photo_by_id(&created_photo.id).await.unwrap();
        assert!(photo.is_some());
        let photo = photo.unwrap();
        assert_eq!(created_photo.id, photo.id);
        assert_eq!(created_photo.image.id, photo.image.id);
    }
    
    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn update_photo() {
        let env: &'static str = env!("DATABASE_URL");
        let pg = PostgresDatabase::connect(env).await.unwrap();

        let owner_user_id = Uuid::new_v4();
        let image_id = Uuid::new_v4();
        let image_url = Url::parse("http://localhost:8080/").unwrap();
        let album_id = uuid::Uuid::new_v4();
        let create_photo = CreatePhoto::new(
            "title",
            "description",
            "category",
            &vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            &owner_user_id,
            &image_id,
            &Some(album_id),
            &Visibility::Private,
            &image_url,
            1024,
            &ImageFormat::Png,
        );

        let created_photo = pg.create_photo(&create_photo).await.unwrap();
        assert_eq!(image_id, created_photo.image.id);

        let new_title = "new_title".to_string();
        let update_photo = UpdatePhoto::new(
            &created_photo.id,
            Some(&new_title),
            None,
        );
        let updated_photo = pg.update_photo(&update_photo).await.unwrap();
        assert_eq!(&updated_photo.id, &created_photo.id);
        assert_eq!(updated_photo.title, new_title);
    }
    
}