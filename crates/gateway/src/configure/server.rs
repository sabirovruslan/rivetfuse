use std::net::{AddrParseError, SocketAddr};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_http_address(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    pub fn socket_address(&self) -> Result<SocketAddr, AddrParseError> {
        self.address().parse()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_server_config_address() {
        let config = ServerConfig {
            host: "localhost".to_string(),
            port: 8080,
        };
        assert_eq!(config.address(), "localhost:8080");
    }
}
