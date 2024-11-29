use secure_photo_hub::setup;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_logging()?;
    let oidc_config = setup::setup().await?;
    setup::init_http_server(oidc_config).await?;

    Ok(())
}

pub fn init_logging() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    log::info!("Logging initialized.");
    Ok(())
}
