//! Runtime configuration.

use std::{env, net::SocketAddr};

use thiserror::Error;

/// Configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Address on which the server accepts connections.
    pub bind_addr: SocketAddr,
    /// Whether browser clients from any origin may issue GET requests.
    pub allow_any_origin: bool,
}

impl Config {
    /// Loads configuration from environment variables.
    ///
    /// `BIND_ADDR` defaults to `127.0.0.1:3000` and `ALLOW_ANY_ORIGIN`
    /// defaults to `false`.
    pub fn from_env() -> Result<Self, ConfigError> {
        let bind_addr = env::var("BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:3000".to_owned())
            .parse()
            .map_err(ConfigError::InvalidBindAddr)?;
        let allow_any_origin = env::var("ALLOW_ANY_ORIGIN")
            .unwrap_or_else(|_| "false".to_owned())
            .parse()
            .map_err(ConfigError::InvalidAllowAnyOrigin)?;

        Ok(Self {
            bind_addr,
            allow_any_origin,
        })
    }
}

/// Errors while reading runtime configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// The bind address could not be parsed.
    #[error("BIND_ADDR must be a socket address: {0}")]
    InvalidBindAddr(#[source] std::net::AddrParseError),
    /// The CORS switch could not be parsed.
    #[error("ALLOW_ANY_ORIGIN must be true or false: {0}")]
    InvalidAllowAnyOrigin(#[source] std::str::ParseBoolError),
}
