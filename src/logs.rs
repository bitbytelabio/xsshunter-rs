use crate::CONFIG;
use std::{fs::File, sync::Arc};
use tracing_subscriber::{filter, prelude::*, EnvFilter};

pub fn start_logging() {
    let filter = EnvFilter::from_default_env().add_directive(
        CONFIG
            .log
            .level
            .parse()
            .unwrap_or_else(|_| tracing::Level::INFO.into()),
    );

    let stdout_log = tracing_subscriber::fmt::layer()
        .with_thread_names(CONFIG.log.tracing.thread_name)
        .with_file(CONFIG.log.tracing.file)
        .with_target(CONFIG.log.tracing.target)
        .with_line_number(CONFIG.log.tracing.line_number);

    let file = File::create(&CONFIG.log.file);
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let log_file = tracing_subscriber::fmt::layer()
        .json()
        .compact()
        .with_thread_names(CONFIG.log.tracing.thread_name)
        .with_file(CONFIG.log.tracing.file)
        .with_target(CONFIG.log.tracing.target)
        .with_line_number(CONFIG.log.tracing.line_number)
        .with_writer(Arc::new(file));

    let metrics_layer = filter::LevelFilter::INFO;

    tracing_subscriber::registry()
        .with(
            stdout_log
                .with_filter(filter)
                .and_then(log_file)
                .with_filter(filter::filter_fn(|metadata| {
                    !metadata.target().starts_with("metrics")
                })),
        )
        .with(metrics_layer.with_filter(filter::filter_fn(|metadata| {
            metadata.target().starts_with("metrics")
        })))
        .init();
}
