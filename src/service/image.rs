use std::io::Cursor;
use std::sync::Arc;
use image::{ImageFormat, ImageReader};
use uuid::Uuid;
use crate::models::service::image::{ImageTransformOptions, Image, ImageTransformation};
use crate::repository::image_reference_repository::ImageReferenceRepository;
use crate::security::auth::user::AuthenticatedUser;
use crate::service::image_storage::ImageStorage;
use crate::service::ImageService;

#[derive(Debug, Clone)]
pub struct ImageServiceImpl<IR, IU: ImageStorage>
    where
        IR: ImageReferenceRepository,
        IU: ImageStorage,
{
    image_reference_repository: Arc<IR>,
    image_uploader: Arc<IU>,
}

impl<IR, IU> ImageServiceImpl<IR, IU>
    where
        IR: ImageReferenceRepository,
        IU: ImageStorage,
{
    pub fn image_reference_repository(&self) -> Arc<IR> {
        self.image_reference_repository.clone()
    }

    pub fn image_uploader(&self) -> Arc<IU> {
        self.image_uploader.clone()
    }

    pub fn new(image_reference_repository: Arc<IR>, image_uploader: Arc<IU>) -> Self {
        Self { image_reference_repository, image_uploader }
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
        
        Ok(Image::new(&image.id(), image.filename(), &image_format, image_bytes, image_size as u32))
    }
}

#[async_trait::async_trait]
impl<IR, IU> ImageService for ImageServiceImpl<IR, IU>
    where
        IR: ImageReferenceRepository,
        IU: ImageStorage,
{
    async fn get_image(
        &self,
        authenticated_user: &AuthenticatedUser,
        id: &Uuid,
        image_transform_options: &ImageTransformOptions
    ) -> anyhow::Result<Option<Image>> {
        let image_reference = self
            .image_reference_repository()
            .find_image_reference_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Image reference not found"))?;
        
        let image = self
            .image_uploader
            .download_image(&image_reference.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Image not found"))?;
        
        Ok(Some(Self::transform_image(image, image_transform_options)?))
    }
}