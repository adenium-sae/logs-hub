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

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: String::from("Logs Hub"),
            env: Environment::Local,
            url: String::from("http://localhost"),
            port: 3000,
            debug: true,
        }
    }
}
