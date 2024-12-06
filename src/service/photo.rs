use aws_sdk_s3::primitives::ByteStream;
use image::ImageFormat;
use url::Url;
use uuid::Uuid;
use crate::models::service::pagination::Page;
use crate::models::service::photo::{CreatePhoto, Photo, UploadPhoto};
use crate::models::service::Visibility;
use crate::PhotoService;
use crate::repository::photo_repository::PhotoRepository;

#[derive(Debug, Clone)]
pub struct Service<R> 
where
    R: PhotoRepository
{
    photo_repository: R,
    aws_s3_client: aws_sdk_s3::Client,
}

impl<R> Service<R> 
where
    R: PhotoRepository
{
    pub fn new(photo_repository: R, aws_s3_client: aws_sdk_s3::Client) -> Self {
        Self {
            photo_repository,
            aws_s3_client,
        }
    }
}

#[async_trait::async_trait]
impl<R> PhotoService for Service<R> 
where 
    R: PhotoRepository 
{
    async fn get_all_photos(&self) -> anyhow::Result<Page<Photo>> {
        let photos = self.photo_repository
            .find_all_photos(30, 0) // TODO: add pagination
            .await?
            .into_iter()
            .map(Photo::from)
            .collect::<Vec<_>>();

        let tot_photos = photos.len();
        Ok(Page::new(photos, 0, tot_photos as u32))
    }

    async fn get_photo_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Photo>> {
        Ok(self.photo_repository
            .find_photo_by_id(id)
            .await?
            .map(Photo::from))
    }

    async fn create_photo(&self, upload_photo: UploadPhoto) -> anyhow::Result<Photo> {
        let image_id = Uuid::new_v4();
        let upload_image = upload_photo.upload_image();
        self.aws_s3_client.put_object()
            .bucket("sample-bucket")
            .key(image_id.to_string())
            .body(ByteStream::from(upload_image.bytes().to_vec()))
            .send()
            .await?;
        
        let owner_user_id = Uuid::new_v4();
        let image_id = Uuid::new_v4();
        let image_url = Url::parse("http://localhost:8080/").unwrap();
        let album_id = uuid::Uuid::new_v4();
        let create_photo = CreatePhoto::new(
            "title".to_string(),
            "description".to_string(),
            "category".to_string(),
            vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            owner_user_id,
            image_id,
            Some(album_id),
            Visibility::Private,
            image_url.clone(),
            1024,
            ImageFormat::Png,
        );
        
        self.photo_repository.create_photo(&create_photo).await.map(Photo::from)
    }
}

#[allow(unused_imports)]
mod tests {
    use actix_web::web::service;
    use crate::repository::PostgresDatabase;
    use super::*;

    #[actix_web::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_all_photos() {
        let env: &'static str = env!("DATABASE_URL");
        let pg = PostgresDatabase::connect(env).await.unwrap();
        // 
        // let service = Service { photo_repository: pg };
        // 
        // dbg!(service.get_all_photos().await);
    }
}