use secure_photo_hub::setup;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let server = setup::spawn_app().await?;
    server.await?;
    Ok(())
}