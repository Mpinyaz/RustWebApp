use clap::Parser;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use tokio::sync::OnceCell;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for the web app.")]
pub struct Cli {
    /// set  the listener hostess
    #[clap(short = 'a', long = "host")]
    pub host: Option<String>,

    /// set the listen port
    #[clap(short = 'p', long = "port")]
    pub port: Option<String>,
}

/// App-configurable settings.
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct AppAddrSettings {
    /// IP to bind to.
    pub host: Option<String>,

    /// Port to listen on.
    pub port: Option<String>,
}
impl AppAddrSettings {
    pub fn new(host: Option<String>, port: Option<String>) -> Self {
        match (host, port) {
            (Some(host), Some(port)) => Self {
                host: Some(host),
                port: Some(port),
            },
            (Some(host), None) => Self {
                host: Some(host),
                port: Self::default().port,
            },
            (None, Some(port)) => Self {
                host: Self::default().host,
                port: Some(port),
            },
            (None, None) => Default::default(),
        }
    }

    pub fn bind_host(&self) -> String {
        format!(
            "{}:{}",
            self.host.as_deref().unwrap_or("0.0.0.0"),
            self.port.as_deref().unwrap_or("3000")
        )
    }

    pub fn is_valid_ip(&self) -> Result<bool, std::net::AddrParseError> {
        match self.host.as_deref() {
            Some(host_str) => match host_str.parse::<Ipv4Addr>() {
                Ok(_) => Ok(true),
                Err(_) => match host_str.parse::<Ipv6Addr>() {
                    Ok(_) => Ok(true),
                    Err(err) => Err(err),
                },
            },
            None => Ok(false),
        }
    }
}
impl Default for AppAddrSettings {
    fn default() -> Self {
        match dotenv() {
            Ok(_) => Self {
                host: Some(std::env::var("APP_HOST").unwrap_or("0.0.0.0".to_string())),
                port: Some(std::env::var("APP_PORT").unwrap_or("3000".to_string())),
            },
            Err(err) => {
                println!("Error loading .env file: {:?}", err);
                Self::default()
            }
        }
    }
}
// Define a struct to represent our server configuration
#[derive(Debug, Clone)]
struct ServerConfig {
    host: String,
    port: String,
}

// Define a struct to represent our database configuration
#[derive(Debug, Clone)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl Config {
    // pub fn from_env() -> Result<Self, dotenv::Error> {
    //     dotenv().ok();
    //
    //     let server = ServerConfig {
    //         host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
    //         port: env::var("PORT").unwrap_or_else(|_| "8080".into()),
    //     };
    //
    //     let database = DatabaseConfig {
    //         url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    //     };
    //
    //     Ok(Self { server, database })
    // }
    pub fn db_url(&self) -> &str {
        &self.database
    }

    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    pub fn server_port(&self) -> &str {
        &self.server.port
    }
}

// Create a static variable to hold our configuration once loaded
pub static CONFIG: OnceCell<Config> = OnceCell::new();

// Asynchronously load our configuration

pub async fn load_config() -> Result<(), dotenv::Error> {
    dotenv().ok();

    let server = ServerConfig {
        host: env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string()),
    };

    let database = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    let config = Config { server, database };

    CONFIG
        .set(config)
        .map_err(|_| dotenv::Error::from(dotenv::errorkind::Msg("Failed to set config")))
}
