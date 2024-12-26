use std::sync::Arc;
use url::Url;
use uuid::Uuid;
use crate::models::service::pagination::Page;
use crate::models::service::photo::{CreatePhoto, Photo, UpdatePhoto, UploadPhoto};
use crate::service::PhotoService;
use crate::service::image_storage::ImageStorage;
use crate::repository::photo_repository::PhotoRepository;
use crate::security::auth::user::AuthenticatedUser;

#[derive(Debug, Clone)]
pub struct PhotoServiceImpl<R, I>
    where
        R: PhotoRepository,
        I: ImageStorage,
{
    photo_repository: Arc<R>,
    image_repository: Arc<I>,
}

impl<R, I> PhotoServiceImpl<R, I>
    where
        R: PhotoRepository,
        I: ImageStorage,
{
    pub fn new(photo_repository: Arc<R>, image_repository: Arc<I>) -> Self {
        Self {
            photo_repository,
            image_repository,
        }
    }
}

#[async_trait::async_trait]
impl<R, I> PhotoService for PhotoServiceImpl<R, I>
    where
        R: PhotoRepository,
        I: ImageStorage,
{
    async fn get_all_photos(&self, _authenticated_user: &AuthenticatedUser) -> anyhow::Result<Page<Photo>> {
        let photos = self.photo_repository
            .find_all_photos(30, 0) // TODO: add pagination
            .await?
            .into_iter()
            .map(Photo::from)
            .collect::<Vec<_>>();

        let tot_photos = photos.len();
        Ok(Page::new(photos, 0, tot_photos as u32))
    }

    async fn get_photo_by_id(
        &self,
        _authenticated_user: &AuthenticatedUser,
        id: &Uuid,
    ) -> anyhow::Result<Option<Photo>> {
        Ok(self.photo_repository
            .find_photo_by_id(id)
            .await?
            .map(Photo::from))
    }

    async fn create_photo(
        &self,
        authenticated_user: &AuthenticatedUser,
        upload_photo: &UploadPhoto,
    ) -> anyhow::Result<Photo> {
        let upload_image = upload_photo.upload_image();
        let (created_image_id, created_image_url) = self.image_repository.upload_image(upload_image.bytes()).await?;

        let create_photo = CreatePhoto::new(
            upload_photo.title(),
            upload_photo.description(),
            upload_photo.category(),
            upload_photo.tags(),
            authenticated_user.id(),
            &created_image_id,
            upload_photo.album_id(),
            upload_photo.visibility(),
            &created_image_url,
            upload_image.size() as u64,
            &upload_image.format(),
        );

        self.photo_repository.create_photo(&create_photo).await.map(Photo::from)
    }

    async fn update_photo(
        &self, 
        authenticated_user: &AuthenticatedUser, 
        update_photo: &UpdatePhoto
    ) -> anyhow::Result<Photo> {
        self.photo_repository
            .update_photo(update_photo)
            .await
            .map(Photo::from)
    }
}

#[allow(unused_imports, dead_code)]
mod tests {
    use actix_web::web::service;
    use image::ImageFormat;

    use crate::models::service::image::{Image, UploadImage};
    use crate::repository::PostgresDatabase;

    use super::*;

    #[derive(Clone)]
    struct MockImageRepository;

    #[async_trait::async_trait]
    impl ImageStorage for MockImageRepository {
        async fn upload_image(&self, _bytes: &[u8]) -> anyhow::Result<(Uuid, url::Url)> {
            Ok((Uuid::new_v4(), Url::parse("https://localhost:8080/").unwrap()))
        }

        async fn download_image(&self, id: &Uuid) -> anyhow::Result<Option<Image>> {
            Ok(None)
        }
    }

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_all_photos() {
        let (photo_service, authenticated_user) = fixtures().await;

        dbg!(photo_service.get_all_photos(&authenticated_user).await.unwrap());
    }

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_create_photo() {
        let (photo_service, authenticated_user) = fixtures().await;

        let upload_photo = UploadPhoto::new(
            "title".to_string(),
            Some(Uuid::new_v4()),
            "description".to_string(),
            "category".to_string(),
            vec!["tag".to_string(), "tag2".to_string()],
            Visibility::Public,
            UploadImage::new(vec![], ImageFormat::Png, 0),
        );

        let created_photo = photo_service.create_photo(&authenticated_user, &upload_photo).await.unwrap();
        let option_photo = photo_service.get_photo_by_id(&authenticated_user, created_photo.id()).await.unwrap();
        assert!(option_photo.is_some());
        assert_eq!(option_photo.unwrap().id(), created_photo.id());
    }

    async fn fixtures() -> (impl PhotoService, AuthenticatedUser) {
        let db_url: &'static str = env!("DATABASE_URL");
        let pg = Arc::new(PostgresDatabase::connect(db_url).await.unwrap());
        let mock_image_repository = Arc::new(MockImageRepository {});
        let service = PhotoServiceImpl {
            photo_repository: pg.clone(),
            image_repository: mock_image_repository.clone(),
        };
        let authenticated_user = AuthenticatedUser::new(
            &Uuid::new_v4(),
            "username_test",
            "test",
            "test",
            "test test",
            "test@test.test",
            true,
        );
        (service, authenticated_user)
    }
}