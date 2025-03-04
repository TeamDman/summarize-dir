pub fn init(filter: tracing::level_filters::LevelFilter) -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(filter.into())
                .from_env()?,
        )
        .without_time()
        .init();
    Ok(())
}
