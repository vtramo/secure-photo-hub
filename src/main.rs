use secure_photo_hub::setup;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    setup::spawn_app().await?;
    Ok(())
}