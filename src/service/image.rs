use std::io::Cursor;
use std::sync::Arc;
use image::{ImageFormat, ImageReader};
use url::Url;
use uuid::Uuid;
use crate::models::service::image::{ImageTransformOptions, Image, ImageTransformation, ImageReference};
use crate::repository::image_reference_repository::ImageReferenceRepository;
use crate::security::auth::user::AuthenticatedUser;
use crate::security::authz::ImagePolicyEnforcer;
use crate::service::image_storage::ImageStorage;
use crate::service::ImageService;

#[derive(Debug, Clone)]
pub struct ImageServiceImpl<IR, IU, IP>
    where
        IR: ImageReferenceRepository,
        IU: ImageStorage,
        IP: ImagePolicyEnforcer,
{
    image_reference_repository: Arc<IR>,
    image_uploader: Arc<IU>,
    image_policy_enforcer: Arc<IP>,
}

impl<IR, IU, IP> ImageServiceImpl<IR, IU, IP>
    where
        IR: ImageReferenceRepository,
        IU: ImageStorage,
        IP: ImagePolicyEnforcer,
{
    pub fn image_reference_repository(&self) -> Arc<IR> {
        self.image_reference_repository.clone()
    }

    pub fn image_uploader(&self) -> Arc<IU> {
        self.image_uploader.clone()
    }

    pub fn new(image_reference_repository: Arc<IR>, image_uploader: Arc<IU>, image_policy_enforcer: Arc<IP>) -> Self {
        Self { image_reference_repository, image_uploader, image_policy_enforcer }
    }
    
    fn transform_image(image: Image, image_transform_options: &ImageTransformOptions) -> anyhow::Result<Image> {
        let mut dyn_image = ImageReader::new(Cursor::new(image.bytes()))
            .with_guessed_format()?
            .decode()?;
        let mut image_format = image.format(); // TODO: add convert to transformation
        
        image_transform_options.transformations()
            .into_iter()
            .for_each(|transformation| match transformation {
                ImageTransformation::HueRotate(huerotate) => dyn_image = dyn_image.huerotate(huerotate),
                ImageTransformation::Thumbnail(nwidth, nheigth) => dyn_image = dyn_image.thumbnail(nwidth, nheigth),
                ImageTransformation::None => {}
            });
        
        let mut image_bytes = Vec::with_capacity(image.bytes().len());
        let image_size = image_bytes.len();
        dyn_image.write_to(&mut Cursor::new(&mut image_bytes), ImageFormat::Png)?;
        
        Ok(Image::new(&image.id(), image.filename(), &image_format, &image.visibility(), image_bytes, image_size as u32))
    }
}

#[async_trait::async_trait]
impl<IR, IU, IP> ImageService for ImageServiceImpl<IR, IU, IP>
    where
        IR: ImageReferenceRepository,
        IU: ImageStorage,
        IP: ImagePolicyEnforcer,
{
    async fn get_image(
        &self,
        authenticated_user: &AuthenticatedUser,
        id: &Uuid,
        image_transform_options: &ImageTransformOptions
    ) -> anyhow::Result<Option<Image>> {
        let image_reference_entity = self
            .image_reference_repository()
            .find_image_reference_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Image reference not found"))?;

        let image_reference = ImageReference::from(image_reference_entity);
        let is_authorized = if image_transform_options.contains_transformations() {
            self.image_policy_enforcer.can_download_then_transform(authenticated_user, &image_reference).await?
        } else {
            self.image_policy_enforcer.can_download(authenticated_user, &image_reference).await?
        };
        if !is_authorized {
            return Err(anyhow::anyhow!("Unauthorized to download image")); // TODO: Error Handling
        }
        
        let image = self
            .image_uploader
            .download_image(image_reference.id())
            .await?
            .ok_or_else(|| anyhow::anyhow!("Image not found"))?;
        
        Ok(Some(Self::transform_image(image, image_transform_options)?))
    }
}

#[derive(Debug, Clone)]
pub struct ImageReferenceUrlBuilder {
    image_by_id_endpoint_url: Url
}

impl ImageReferenceUrlBuilder {
    pub fn new(image_by_id_endpoint_url: &Url) -> Self {
        Self { image_by_id_endpoint_url: image_by_id_endpoint_url.clone() }
    }
    
    pub fn build(&self, image_reference_id: &Uuid) -> Url {
        self.image_by_id_endpoint_url
            .clone()
            .join(&image_reference_id.to_string())
            .unwrap()
    } 
}