use aws_sdk_s3::primitives::ByteStream;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ImageRepository: Clone + Send + Sync + 'static {
    async fn save_image(&self, bytes: &[u8]) -> anyhow::Result<Uuid>;
}

#[async_trait::async_trait]
impl ImageRepository for aws_sdk_s3::Client {
    async fn save_image(&self, bytes: &[u8]) -> anyhow::Result<Uuid> {
        let image_id = Uuid::new_v4();
        
        self.put_object()
            .bucket("sample-bucket") // TODO
            .key(image_id.to_string())
            .body(ByteStream::from(bytes.to_vec()))
            .send()
            .await?;
        
        Ok(image_id)
    }
}