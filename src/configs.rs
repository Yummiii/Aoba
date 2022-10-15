use figment::{providers::Env, Figment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configs {
    pub server_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Default for Configs {
    fn default() -> Self {
        Self {
            server_port: 3000,
            database_url: String::from("mysql://root:root@localhost:3306/Aoba"),
            jwt_secret: String::from("please-change-me"),
        }
    }
}

impl Configs {
    pub fn get_config() -> Self {
        let config = Figment::new().join(Env::prefixed("AOBA_")).extract();
        match config {
            Ok(config) => {
                log::info!("Settings loaded successfully");
                config
            }
            Err(_) => {
                log::warn!("Failed loading settings, using defaults");
                Configs::default()
            }
        }
    }
}
