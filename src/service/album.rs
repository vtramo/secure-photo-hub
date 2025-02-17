use std::sync::Arc;
use uuid::Uuid;
use crate::models::service::album::{Album, CreateAlbum, CreateAlbumWithCover, UpdateAlbum};
use crate::models::service::pagination::Page;
use crate::repository::album_repository::AlbumRepository;
use crate::security::auth::user::AuthenticatedUser;
use crate::security::authz::AlbumPolicyEnforcer;
use crate::service::AlbumService;
use crate::service::image::ImageReferenceUrlBuilder;
use crate::service::image_storage::ImageStorage;

#[derive(Debug, Clone)]
pub struct AlbumServiceImpl<R, I, P>
    where
        R: AlbumRepository,
        I: ImageStorage,
        P: AlbumPolicyEnforcer,
{
    album_repository: Arc<R>,
    image_repository: Arc<I>,
    album_policy_enforcer: Arc<P>,
    image_reference_url_builder: Arc<ImageReferenceUrlBuilder>,
}

impl<R, I, P> AlbumServiceImpl<R, I, P> 
    where
        R: AlbumRepository,
        I: ImageStorage,
        P: AlbumPolicyEnforcer,
{
    pub fn album_repository(&self) -> Arc<R> {
        self.album_repository.clone()
    }
    pub fn new(
        album_repository: Arc<R>, 
        image_repository: Arc<I>, 
        album_policy_enforcer: Arc<P>,
        image_reference_url_builder: Arc<ImageReferenceUrlBuilder>,
    ) -> Self {
        Self { album_repository, image_repository, album_policy_enforcer, image_reference_url_builder }
    }
}

#[async_trait::async_trait]
impl<R, I, P> AlbumService for AlbumServiceImpl<R, I, P>
    where
        R: AlbumRepository,
        I: ImageStorage,
        P: AlbumPolicyEnforcer,
{
    async fn get_all_albums(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<Page<Album>> {
        let albums = self.album_repository()
            .find_all_albums()
            .await?
            .into_iter()
            .map(Album::from)
            .collect::<Vec<_>>();

        let albums = self.album_policy_enforcer.filter_albums_by_view_permission(authenticated_user, albums).await?;

        let tot_albums = albums.len();
        Ok(Page::new(albums, 0, tot_albums as u32))
    }

    async fn get_album_by_id(&self, authenticated_user: &AuthenticatedUser, id: &Uuid) -> anyhow::Result<Option<Album>> {
        let album_option = self
            .album_repository
            .find_album_by_id(id)
            .await?
            .map(Album::from);

        if let Some(photo) = &album_option {
            let can_view_album = self.album_policy_enforcer.can_view_album(authenticated_user, photo).await?;
            if !can_view_album {
                return Err(anyhow::anyhow!("Unauthorized to view album with id: {}", id).into()); // TODO: Error Handling
            }
        }

        Ok(album_option)
    }

    async fn create_album(
        &self,
        authenticated_user: &AuthenticatedUser,
        create_album_with_cover: &CreateAlbumWithCover
    ) -> anyhow::Result<Album> {
        let can_edit_album = self.album_policy_enforcer.can_create_album(authenticated_user).await?;
        if !can_edit_album {
            return Err(anyhow::anyhow!("Unauthorized to create album")); // TODO: Error Handling
        }
        
        let upload_cover_image = create_album_with_cover.upload_image();
        let (created_cover_image_id, created_cover_image_url) = self.image_repository.upload_image(upload_cover_image).await?;
        let cover_image_reference_url = self.image_reference_url_builder.build(&created_cover_image_id);
        
        let create_album = CreateAlbum::new(
            create_album_with_cover.title().to_string(),
            create_album_with_cover.description().to_string(),
            create_album_with_cover.visibility().clone(),
            authenticated_user.id().clone(),
            created_cover_image_id,
            created_cover_image_url,
            cover_image_reference_url,
            upload_cover_image.size() as u64,
            upload_cover_image.format(),
        );
        
        self.album_repository()
            .create_album(&create_album)
            .await
            .map(Album::from)
    }

    async fn update_album(
        &self, 
        authenticated_user: &AuthenticatedUser, 
        update_album: &UpdateAlbum
    ) -> anyhow::Result<Album> {
        let album_id = update_album.id();
        let album = self.album_repository
            .find_album_by_id(album_id)
            .await?
            .map(Album::from)
            .ok_or(anyhow::anyhow!("Not found"))?;
        
        let can_edit_album = self.album_policy_enforcer.can_edit_album(authenticated_user, &album, update_album).await?;
        if !can_edit_album {
            return Err(anyhow::anyhow!("Unauthorized to edit album with id {}", update_album.id()).into()); // TODO: Error Handling
        }
        
        self.album_repository
            .update_album(update_album)
            .await
            .map(Album::from)
    }
}
