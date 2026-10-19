//! Service entry point.

use std::{env, error::Error, io};

use rust_starter::{app, config::Config};
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let environment = match env::var("APP_ENV") {
        Ok(value) if !value.trim().is_empty() => value.trim().to_ascii_lowercase(),
        _ => "development".to_owned(),
    };
    let env_file = match environment.as_str() {
        "development" => ".env.development",
        "production" => ".env.production",
        other => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("APP_ENV must be development or production, got {other:?}"),
            )
            .into());
        }
    };
    let env_file_loaded = match dotenvy::from_path(env_file) {
        Ok(()) => true,
        Err(error) if error.not_found() => false,
        Err(error) => return Err(format!("failed to load {env_file}: {error}").into()),
    };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with_target(false)
        .init();
    if env_file_loaded {
        info!(%environment, env_file, "loaded environment file");
    } else {
        warn!(%environment, env_file, "environment file not found; using process environment and defaults");
    }

    let config = Config::from_env()?;
    let listener = TcpListener::bind(config.bind_addr).await?;
    info!(address = %config.bind_addr, "server listening");

    axum::serve(listener, app(&config))
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    info!("shutdown signal received");
}
