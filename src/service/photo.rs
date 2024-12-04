use crate::models::service::pagination::Page;
use crate::models::service::photo::Photo;
use crate::PhotoService;
use crate::repository::photo_repository::PhotoRepository;

#[derive(Debug, Clone)]
pub struct Service<R> 
where
    R: PhotoRepository
{
    photo_repository: R,
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

        let service = Service { photo_repository: pg };

        dbg!(service.get_all_photos().await);
    }
}