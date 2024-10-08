//! TODO.
//!

use clap::Parser;

/// Command-line arguments.
#[derive(Debug, Clone, Parser)]
pub struct Args {
    /// Bound server port.
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,

    /// Database connection string.
    #[arg(short, long, default_value = "postgresql://usr:pwd@localhost:5432/db")]
    pub database: String,

    /// Bound (redirect) server port.
    #[cfg(feature = "support-https")]
    #[arg(short, long, default_value_t = 3001)]
    pub redirect: u16,

    /// Directory containing `cert.pem` and `key.pem` files.
    #[cfg(feature = "support-https")]
    #[arg(short, long)]
    pub keys: std::path::PathBuf,
}
