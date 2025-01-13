use anyhow::Context;
use uuid::Uuid;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use crate::models::service::image::{Image, UploadImage};
use crate::models::service::Visibility;
use crate::setup::AwsS3Config;

#[async_trait::async_trait]
pub trait ImageStorage: Clone + Send + Sync + 'static {
    async fn upload_image(&self, upload_image: &UploadImage) -> anyhow::Result<(Uuid, url::Url)>;
    async fn download_image(&self, id: &Uuid) -> anyhow::Result<Option<Image>>;
}

#[derive(Clone, Debug)]
pub struct AwsS3Client {
    aws_sdk_s3: aws_sdk_s3::Client,
    bucket_name: String,
    endpoint_url: String,
}

#[async_trait::async_trait]
impl ImageStorage for AwsS3Client {
    async fn upload_image(&self, upload_image: &UploadImage) -> anyhow::Result<(Uuid, url::Url)> {
        let image_id = self.put_object_image(upload_image).await?;
        let resource_url = self.build_resource_url(&image_id);

        Ok((image_id, resource_url))
    }

    async fn download_image(&self, id: &Uuid) -> anyhow::Result<Option<Image>> {
        let object = self.aws_sdk_s3
            .get_object()
            .bucket(&self.bucket_name)
            .key(id.to_string())
            .send()
            .await
            .context("Failed to download image from S3")?; // TODO: error handling

        let metadata = object.metadata
            .as_ref()
            .and_then(|metadata| metadata.get(Self::IMAGE_METADATA_KEY))
            .ok_or_else(|| anyhow::anyhow!("Image metadata not found for key: {}", Self::IMAGE_METADATA_KEY))?;  // TODO: error handling

        let image_metadata: ImageMetadata = serde_json::from_str(metadata)
            .context("Failed to deserialize image metadata")?; // TODO: error handling

        let bytes = object.body.collect().await?.into_bytes().to_vec();

        Ok(Some(Image::new(
            id,
            &image_metadata.filename,
            &image_metadata.format,
            &image_metadata.visibility,
            bytes,
            image_metadata.size as u32,
        )))
    }
}

impl AwsS3Client {
    const IMAGE_METADATA_KEY: &'static str = "image_metadata";

    pub fn new(aws_s3config: &AwsS3Config) -> Self {
        let endpoint_url = Self::strip_https_scheme_prefix(aws_s3config);
        Self {
            aws_sdk_s3: Client::new(&aws_s3config.sdk_config),
            bucket_name: aws_s3config.bucket_name.clone(),
            endpoint_url
        }
    }

    fn strip_https_scheme_prefix(aws_s3config: &AwsS3Config) -> String {
        // TODO: fix
        aws_s3config.sdk_config
            .endpoint_url()
            .unwrap()
            .to_string()
            .strip_prefix("https://")
            .unwrap()
            .to_string()
    }

    async fn put_object_image(&self, upload_image: &UploadImage) -> anyhow::Result<Uuid> {
        let image_id = Uuid::new_v4();
        let key = image_id.to_string();
        let image_metadata = serde_json::to_string(&ImageMetadata::from(upload_image))?;

        self.aws_sdk_s3
            .put_object()
            .bucket(&self.bucket_name)
            .key(&key)
            .body(ByteStream::from(upload_image.bytes().to_vec()))
            .metadata(Self::IMAGE_METADATA_KEY, image_metadata)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!(
                "Failed to upload object with key '{}' to bucket '{}': {:?}",
                key, self.bucket_name, e
            ))?;

        Ok(image_id)
    }

    fn build_resource_url(&self, key: &Uuid) -> url::Url {
        url::Url::parse(&format!("https://{}.{}/{}", self.bucket_name, self.endpoint_url, key)).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageMetadata {
    pub filename: String,
    pub format: ImageFormat,
    pub visibility: Visibility,
    pub size: usize,
}

impl From<&UploadImage> for ImageMetadata {
    fn from(upload_image: &UploadImage) -> Self {
        Self {
            filename: upload_image.filename().to_string(),
            format: upload_image.format(),
            visibility: upload_image.visibility(),
            size: upload_image.size(),
        }
    }
}