use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ascii_art: Option<String>,
    pub modules: Option<Vec<String>>,
    pub colors: Option<ColorConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ColorConfig {
    pub title: Option<String>,
    #[allow(dead_code)]
    pub value: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ascii_art: None,
            modules: None,
            colors: Some(ColorConfig {
                title: Some("blue".to_string()),
                value: Some("cyan".to_string()),
            }),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("betterfetch")
            .join("config.toml");

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path).unwrap_or_else(|_| {
                eprintln!("Failed to read config file, using defaults.");
                String::new()
            });

            toml::from_str(&contents).unwrap_or_else(|err| {
                eprintln!("Invalid config format: {}. Using defaults.", err);
                Config::default()
            })
        } else {
            Config::default()
        }
    }
}
