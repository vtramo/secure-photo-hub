use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use uuid::Uuid;
use crate::setup::AwsS3Config;

#[async_trait::async_trait]
pub trait ImageRepository: Clone + Send + Sync + 'static {
    async fn save_image(&self, bytes: &[u8]) -> anyhow::Result<(Uuid, url::Url)>;
}

#[derive(Clone, Debug)]
pub struct AwsS3Client {
    aws_sdk_s3: aws_sdk_s3::Client,
    bucket_name: String,
    endpoint_url: String,
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

#[async_trait::async_trait]
impl ImageRepository for AwsS3Client {
    async fn save_image(&self, bytes: &[u8]) -> anyhow::Result<(Uuid, url::Url)> {
        let image_id = Uuid::new_v4();
        let image_id_string = image_id.to_string();

        self.put_object(&image_id_string, ByteStream::from(bytes.to_vec())).await?;
        let resource_url = self.build_resource_url(&image_id_string);

        Ok((image_id, resource_url))
    }
}