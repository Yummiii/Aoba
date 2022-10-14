
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_port: 3000,
            database_url: String::from("mysql://root:root@localhost:3306/Aoba"),
            jwt_secret: String::from("please-change-me"),
        }
    }
}

impl Config {
    pub fn get_config() -> Self {
        let config = Figment::new()
            .merge(Toml::file("Aoba.toml"))
            .join(Env::prefixed("AOBA_"))
            .extract();

        match config {
            Ok(config) => {
                log::info!("Config loaded successfully");
                config
            }
            Err(_) => {
                log::warn!("Config loading failed, using default config");
                Config::default()
            }
        }
    }
}