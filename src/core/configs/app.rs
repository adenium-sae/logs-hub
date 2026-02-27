use dotenvy::dotenv;
use std::env;

enum Environment {
    Local,
    Staging,
    Production,
}

pub struct AppConfig {
    pub name: String,
    pub env: Environment,
    pub url: String,
    pub port: u16,
    pub debug: bool,
}

impl AppConfig {
    pub fn load() -> Self {
        dotenv().ok();
        let name = env::var("APP_NAME").unwrap_or_else(|_| "Logs Hub".to_string());
        let env_str = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());
        let env = match env_str.as_str() {
            "production" => Environment::Production,
            "staging" => Environment::Staging,
            _ => Environment::Local,
        };
        let url = env::var("APP_URL").unwrap_or_else(|_| "http://localhost".to_string());
        let port_str = env::var("PORT").expect("🚨 ERROR: Variable PORT not found in .env");
        let port: u16 = port_str.parse().expect("🚨 ERROR: PORT must be a valid number");
        let debug_str = env::var("APP_DEBUG").unwrap_or_else(|_| "false".to_string());
        let debug = debug_str == "true";
        Self {
            name,
            env,
            url,
            port,
            debug,
        }
    }
}