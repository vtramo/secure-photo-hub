pub fn init_logging() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    log::info!("Logging initialized.");
    Ok(())
}