use std::sync::Arc;
use url::Url;
use uuid::Uuid;
use crate::models::service::pagination::Page;
use crate::models::service::photo::{CreatePhoto, Photo, UpdatePhoto, UploadPhoto};
use crate::service::PhotoService;
use crate::service::image_storage::ImageStorage;
use crate::repository::photo_repository::PhotoRepository;
use crate::security::auth::user::AuthenticatedUser;
use crate::security::authz::PhotoPolicyEnforcer;
use crate::service::image::ImageReferenceUrlBuilder;

#[derive(Debug, Clone)]
pub struct PhotoServiceImpl<R, I, P>
    where
        R: PhotoRepository,
        I: ImageStorage,
        P: PhotoPolicyEnforcer,
{
    photo_repository: Arc<R>,
    image_repository: Arc<I>,
    photo_policy_enforcer: Arc<P>,
    image_reference_url_builder: Arc<ImageReferenceUrlBuilder>,
}

impl<R, I, P> PhotoServiceImpl<R, I, P>
    where
        R: PhotoRepository,
        I: ImageStorage,
        P: PhotoPolicyEnforcer,
{
    pub fn new(
        photo_repository: Arc<R>, 
        image_repository: Arc<I>, 
        photo_policy_enforcer: Arc<P>,
        image_reference_url_builder: Arc<ImageReferenceUrlBuilder>
    ) -> Self {
        Self {
            photo_repository,
            image_repository,
            photo_policy_enforcer,
            image_reference_url_builder
        }
    }
}

#[async_trait::async_trait]
impl<R, I, P> PhotoService for PhotoServiceImpl<R, I, P>
    where
        R: PhotoRepository,
        I: ImageStorage,
        P: PhotoPolicyEnforcer
{
    async fn get_all_photos(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<Page<Photo>> {
        let photos = self.photo_repository
            .find_all_photos(30, 0) // TODO: add pagination
            .await?
            .into_iter()
            .map(Photo::from)
            .collect::<Vec<_>>();

        let photos = self.photo_policy_enforcer.filter_photos_by_view_permission(authenticated_user, photos).await?;

        let tot_photos = photos.len();
        Ok(Page::new(photos, 0, tot_photos as u32))
    }

    async fn get_photo_by_id(
        &self,
        authenticated_user: &AuthenticatedUser,
        id: &Uuid,
    ) -> anyhow::Result<Option<Photo>> {
        let photo_option = self
            .photo_repository
            .find_photo_by_id(id)
            .await?
            .map(Photo::from);
        
        if let Some(photo) = &photo_option {
            let can_view_photo = self.photo_policy_enforcer.can_view_photo(authenticated_user, photo).await?;
            if !can_view_photo {
                return Err(anyhow::anyhow!("Unauthorized to view photo with id: {}", id).into()); // TODO: Error Handling
            }
        }
        
        Ok(photo_option)
    }

    async fn create_photo(
        &self,
        authenticated_user: &AuthenticatedUser,
        upload_photo: &UploadPhoto,
    ) -> anyhow::Result<Photo> {
        let can_create_photo = self.photo_policy_enforcer.can_create_photo(authenticated_user).await?;
        if !can_create_photo {
            return Err(anyhow::anyhow!("Unauthorized to create a photo").into()); // TODO: Error Handling
        }

        let upload_image = upload_photo.upload_image();
        let (created_image_id, created_image_url) = self.image_repository.upload_image(upload_image).await?;
        let image_reference_url = self.image_reference_url_builder.build(&created_image_id);

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
            &image_reference_url,
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
        let photo_id = update_photo.id();
        
        let photo = self.photo_repository
            .find_photo_by_id(&photo_id)
            .await?
            .map(Photo::from)
            .ok_or(anyhow::anyhow!("Not found"))?;
        
        let can_edit_photo = self.photo_policy_enforcer.can_edit_photo(authenticated_user, &photo, update_photo).await?;
        if !can_edit_photo {
            return Err(anyhow::anyhow!("Unauthorized to edit photo with id {}", update_photo.id()).into()); // TODO: Error Handling
        }
        
        self.photo_repository
            .update_photo(update_photo)
            .await
            .map(Photo::from)
    }
}

#[allow(unused_imports, dead_code)]
mod tests {
    use actix_web::web::service;
    use async_trait::async_trait;
    use image::ImageFormat;

    use crate::models::service::image::{Image, UploadImage};
    use crate::models::service::Visibility;
    use crate::repository::PostgresDatabase;
    use crate::security::auth::oauth::OAuthAccessTokenHolder;

    use super::*;

    #[derive(Clone)]
    struct MockImageRepository;

    #[derive(Clone)]
    struct MockPhotoPolicyEnforcer;

    #[async_trait::async_trait]
    impl ImageStorage for MockImageRepository {
        async fn upload_image(&self, _bytes: &UploadImage) -> anyhow::Result<(Uuid, url::Url)> {
            Ok((Uuid::new_v4(), Url::parse("https://localhost:8080/").unwrap()))
        }

        async fn download_image(&self, _id: &Uuid) -> anyhow::Result<Option<Image>> {
            Ok(None)
        }
    }

    #[async_trait()]
    impl OAuthAccessTokenHolder for MockPhotoPolicyEnforcer {
        async fn get_access_token(&self) -> anyhow::Result<String> {
            Ok(":)".to_string())
        }
    }

    #[async_trait::async_trait]
    impl PhotoPolicyEnforcer for MockPhotoPolicyEnforcer {
        async fn can_view_photo(&self, _authenticated_user: &AuthenticatedUser, _photo: &Photo) -> anyhow::Result<bool> {
            Ok(true)
        }

        async fn can_create_photo(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<bool> {
            Ok(true)
        }

        async fn can_edit_photo(&self, _authenticated_user: &AuthenticatedUser, _photo: &Photo, _update_photo: &UpdatePhoto) -> anyhow::Result<bool> {
            Ok(true)
        }

        async fn filter_photos_by_view_permission<'a>(&self, authenticated_user: &AuthenticatedUser, photos: Vec<Photo>) -> anyhow::Result<Vec<Photo>> {
            Ok(photos)
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
            UploadImage::new("", vec![], ImageFormat::Png, Visibility::Private, 0),
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
        let mock_photo_policy_enforcer = Arc::new(MockPhotoPolicyEnforcer {});
        let mock_image_reference_url_builder = Arc::new(ImageReferenceUrlBuilder::new(&Url::parse("http://localhost:8080/images/").unwrap()));
        let service = PhotoServiceImpl {
            photo_repository: pg.clone(),
            image_repository: mock_image_repository.clone(),
            photo_policy_enforcer: mock_photo_policy_enforcer,
            image_reference_url_builder: mock_image_reference_url_builder
        };
        let authenticated_user = AuthenticatedUser::new(
            &Uuid::new_v4(),
            "username_test",
            "test",
            "test",
            "test test",
            "test@test.test",
            true,
            "token"
        );
        (service, authenticated_user)
    }
}