use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use super::env::LoggingConfig;

pub fn init_logging(config: &LoggingConfig) {
    let file_appender = tracing_appender::rolling::daily(&config.directory, &config.file_name);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new(&config.level)).unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true)
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(true),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(true),
        )
        .with(env_filter)
        .init();

    tracing::info!("📝 Logging initialized");
    tracing::info!("📁 Log directory: {}", config.directory);
    tracing::info!("📄 Log file: {}", config.file_name);
    tracing::info!("📊 Log level: {}", config.level);
}
