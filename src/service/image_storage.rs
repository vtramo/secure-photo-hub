use uuid::Uuid;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use image::ImageFormat;
use crate::models::service::image::Image;
use crate::setup::AwsS3Config;

#[async_trait::async_trait]
pub trait ImageStorage: Clone + Send + Sync + 'static {
    async fn upload_image(&self, bytes: &[u8]) -> anyhow::Result<(Uuid, url::Url)>;
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
    async fn upload_image(&self, bytes: &[u8]) -> anyhow::Result<(Uuid, url::Url)> {
        let image_id = Uuid::new_v4();
        let image_id_string = image_id.to_string();

        self.put_object(&image_id_string, ByteStream::from(bytes.to_vec())).await?;
        let resource_url = self.build_resource_url(&image_id_string);

        Ok((image_id, resource_url))
    }

    async fn download_image(&self, id: &Uuid) -> anyhow::Result<Option<Image>> {
        let object = self.aws_sdk_s3
            .get_object()
            .bucket(&self.bucket_name)
            .key(id.to_string())
            .send()
            .await?;

        dbg!(&object);
        let bytes = object.body.collect().await?.into_bytes().to_vec();
        let size = bytes.len() as u32;
        Ok(Some(Image::new(id, &ImageFormat::Jpeg, bytes, size)))
    }
}

impl AwsS3Client {
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

    async fn put_object(&self, key: &str, byte_stream: ByteStream) -> anyhow::Result<()> {
        self.aws_sdk_s3
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(byte_stream)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!(
                "Failed to upload object with key '{}' to bucket '{}': {:?}",
                key, self.bucket_name, e
            ))
    }

    fn build_resource_url(&self, key: &str) -> url::Url {
        url::Url::parse(&format!("https://{}.{}/{}", self.bucket_name, self.endpoint_url, key)).unwrap()
    }
}
