use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub environment: Environment,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub directory: String,
    pub file_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
    Staging,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let server = ServerConfig {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().map_err(|_| ConfigError::InvalidPort)?,
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()).parse().unwrap_or(Environment::Development),
        };

        let database = DatabaseConfig {
            url: env::var("DATABASE_URL").map_err(|_| ConfigError::MissingDatabaseUrl)?,
            max_connections: env::var("DB_MAX_CONNECTIONS").unwrap_or_else(|_| "10".to_string()).parse().unwrap_or(10),
            min_connections: env::var("DB_MIN_CONNECTIONS").unwrap_or_else(|_| "2".to_string()).parse().unwrap_or(2),
            connect_timeout: env::var("DB_CONNECT_TIMEOUT").unwrap_or_else(|_| "30".to_string()).parse().unwrap_or(30),
        };

        let logging = LoggingConfig {
            level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            directory: env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string()),
            file_name: env::var("LOG_FILE").unwrap_or_else(|_| "server.log".to_string()),
        };

        let jwt = JwtConfig {
            secret: env::var("JWT_SECRET").map_err(|_| ConfigError::MissingJwtSecret)?,
            expiration: env::var("JWT_EXPIRATION").unwrap_or_else(|_| "86400".to_string()).parse().unwrap_or(86400),
        };

        Ok(Config { server, database, logging, jwt })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    #[allow(dead_code)]
    pub fn is_production(&self) -> bool {
        self.server.environment == Environment::Production
    }

    #[allow(dead_code)]
    pub fn is_development(&self) -> bool {
        self.server.environment == Environment::Development
    }
}

impl std::str::FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Environment::Development),
            "production" | "prod" => Ok(Environment::Production),
            "staging" | "stage" => Ok(Environment::Staging),
            _ => Err(format!("Unknown environment: {}", s)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid port number")]
    InvalidPort,
    #[error("DATABASE_URL environment variable is required")]
    MissingDatabaseUrl,
    #[error("JWT_SECRET environment variable is required")]
    MissingJwtSecret,
}
