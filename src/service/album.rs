use std::sync::Arc;
use uuid::Uuid;
use crate::models::service::album::{Album, CreateAlbum, CreateAlbumWithCover};
use crate::models::service::pagination::Page;
use crate::repository::album_repository::AlbumRepository;
use crate::security::auth::user::AuthenticatedUser;
use crate::service::AlbumService;
use crate::service::image_storage::ImageStorage;

#[derive(Debug, Clone)]
pub struct AlbumServiceImpl<R, I>
    where
        R: AlbumRepository,
        I: ImageStorage,
{
    album_repository: Arc<R>,
    image_repository: Arc<I>,
}

impl<R, I> AlbumServiceImpl<R, I> 
    where
        R: AlbumRepository,
        I: ImageStorage,
{
    pub fn album_repository(&self) -> Arc<R> {
        self.album_repository.clone()
    }
    pub fn new(album_repository: Arc<R>, image_repository: Arc<I>) -> Self {
        Self { album_repository, image_repository }
    }
}

#[async_trait::async_trait]
impl<R, I> AlbumService for AlbumServiceImpl<R, I>
    where
        R: AlbumRepository,
        I: ImageStorage,
{
    async fn get_all_albums(&self, _authenticated_user: &AuthenticatedUser) -> anyhow::Result<Page<Album>> {
        let albums = self.album_repository()
            .find_all_albums()
            .await?
            .into_iter()
            .map(Album::from)
            .collect::<Vec<_>>();

        let tot_albums = albums.len();
        Ok(Page::new(albums, 0, tot_albums as u32))
    }

    async fn get_album_by_id(&self, _authenticated_user: &AuthenticatedUser, id: &Uuid) -> anyhow::Result<Option<Album>> {
        Ok(self.album_repository()
            .find_album_by_id(id)
            .await?
            .map(Album::from))
    }

    async fn create_album(
        &self,
        authenticated_user: &AuthenticatedUser,
        create_album_with_cover: &CreateAlbumWithCover
    ) -> anyhow::Result<Album> {
        let upload_cover_image = create_album_with_cover.upload_image();
        let (created_image_id, created_image_url) = self.image_repository.upload_image(upload_cover_image.bytes()).await?;
        
        let create_album = CreateAlbum::new(
              create_album_with_cover.title().to_string(),
              create_album_with_cover.description().to_string(),
              create_album_with_cover.visibility().clone(),
              authenticated_user.id().clone(),
              created_image_id,
              created_image_url,
              upload_cover_image.size() as u64,
              upload_cover_image.format(),
        );
        
        self.album_repository()
            .create_album(&create_album)
            .await
            .map(Album::from)
    }
}
