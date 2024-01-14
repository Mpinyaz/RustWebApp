use clap::Parser;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
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
