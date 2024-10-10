//! TODO.
//!

mod load_json;
mod load_toml;
mod load_yaml;

use std::ffi::OsStr;
use std::path::PathBuf;

use axiston_server::service::AppConfig;
use clap::Parser;
use serde::Deserialize;

use crate::config::load_json::load_json;
use crate::config::load_toml::load_toml;
use crate::config::load_yaml::load_yaml;
use crate::server::ServerConfig;

/// Command-line arguments.
#[derive(Debug, Clone, Parser, Deserialize)]
#[must_use = "configs do nothing unless you use them"]
pub struct Args {
    /// Bound server port.
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
    /// Bound (redirect) server port.
    #[cfg(feature = "support-https")]
    #[arg(short, long, default_value_t = 3001)]
    pub redirect: u16,
    /// Directory containing `cert.pem` and `key.pem` files.
    #[cfg(feature = "support-https")]
    #[arg(short, long)]
    pub keys: PathBuf,

    /// Database connection string.
    #[arg(short, long, default_value = "postgresql://usr:pwd@localhost:5432/db")]
    pub database: String,
    /// Enables database connection settings for multiple gateways.
    #[arg(short, long, default_value_t = false)]
    pub multiple: bool,
}

/// Commands for the CLI.
#[derive(Debug, Clone, Parser)]
#[must_use = "configs do nothing unless you use them"]
pub enum Cli {
    /// Provide configuration via command-line flags.
    Flags {
        #[command(flatten)]
        args: Args,
    },
    /// Provide configuration via a configuration file.
    File {
        /// Path to the configuration file.
        #[arg(short, long, value_name = "FILE")]
        path: PathBuf,
    },
}

impl Args {
    /// Parses the provided configuration:
    /// - via command-line flags or
    /// - via a configuration file.
    pub fn load() -> anyhow::Result<Self> {
        match Cli::parse() {
            Cli::Flags { args } => Ok(args),
            Cli::File { path } => match path.extension() {
                Some(ext) if OsStr::new("toml") == ext => load_toml(path),
                Some(ext) if OsStr::new("json") == ext => load_json(path),
                Some(ext) if OsStr::new("yaml") == ext => load_yaml(path),
                _ => Err(anyhow::anyhow!("should specify a supported file extension")),
            },
        }
    }

    /// Returns a new [`AppConfig`].
    pub fn build_app_config(&self) -> AppConfig {
        AppConfig {
            database_conn: self.database.clone(),
            multiple_gateways: self.multiple,
        }
    }

    /// Returns a new [`ServerConfig`].
    pub fn build_server_config(&self) -> ServerConfig {
        ServerConfig {
            port: self.port,
            #[cfg(feature = "support-https")]
            redirect: self.redirect,
            #[cfg(feature = "support-https")]
            cert: self.keys.join("./cert.pem"),
            #[cfg(feature = "support-https")]
            key: self.keys.join("./key.pem"),
        }
    }
}
