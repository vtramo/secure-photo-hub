use anyhow::{anyhow, Context};
use sqlx::{PgConnection, query_file_as};
use uuid::Uuid;

use crate::models::entity::{ImageFormatEntity, ImageReferenceEntity, VisibilityEntity};
use crate::models::service::image::ImageReference;
use crate::repository::PostgresDatabase;

#[async_trait::async_trait]
pub trait ImageReferenceRepository: Clone + Send + Sync + 'static {
    async fn find_image_reference_by_id(&self, id: &Uuid) -> anyhow::Result<Option<ImageReferenceEntity>>;
}


#[async_trait::async_trait]
impl ImageReferenceRepository for PostgresDatabase {

    async fn find_image_reference_by_id(&self, id: &Uuid) -> anyhow::Result<Option<ImageReferenceEntity>> {
        let mut conn = self.acquire()
            .await
            .with_context(|| "Unable to acquire a database connection".to_string())?;
        
        let image_reference_entity: Option<ImageReferenceEntity> = query_file_as!(
            ImageReferenceEntity,
            "queries/postgres/find_image_reference_by_id.sql",
            id
        ).fetch_optional(&mut *conn)
        .await?;
        
        Ok(image_reference_entity)
    }
}

impl PostgresDatabase {
    pub async fn insert_image_reference(
        image_reference: &ImageReference,
        conn: &mut PgConnection,
    ) -> anyhow::Result<ImageReferenceEntity> {
        let id = image_reference.id();
        let owner_user_id = image_reference.owner_user_id();
        let url = image_reference.url().to_string();
        let size = image_reference.size();
        let format = ImageFormatEntity::from(image_reference.format());
        let visibility = VisibilityEntity::from(image_reference.visibility());
        
        let created_image: ImageReferenceEntity = query_file_as!(
            ImageReferenceEntity,
            "queries/postgres/insert_image_reference.sql",
            id,
            owner_user_id,
            visibility as _,
            url,
            size as i64,
            format as _
        ).fetch_all(conn)
            .await?
            .get(0)
            .cloned()
            .take()
            .ok_or(anyhow!("Unable to insert an image"))?;

        Ok(created_image)
    }
}