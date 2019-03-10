#![allow(dead_code)]
#![allow(unused_variables)]

use serde_derive::Deserialize;
use std::collections::BTreeMap;
use toml;

use crate::error::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub exchange: BTreeMap<String, APIConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct APIConfig {
    pub api_key: String,                // todo: optional
    pub secret_key: String,             // todo: optional
    pub positions: Option<Vec<String>>, // todo: remove
    pub watch: Option<Vec<String>>,     // todo: remove
}

pub fn read() -> CoreResult<Config> {
    pub fn file_exists(path: &str) -> bool {
        use std::fs;

        match fs::metadata(path) {
            Ok(p) => p.is_file(),
            Err(_) => false,
        }
    }

    fn str_from_file_path(path: &str) -> CoreResult<String> {
        use std::io::prelude::*;

        let mut handle = ::std::fs::File::open(path)?;
        let mut bytebuffer = Vec::new();

        handle.read_to_end(&mut bytebuffer)?;

        Ok(String::from_utf8(bytebuffer)?)
    }

    let home_path =
        dirs::home_dir().ok_or_else(|| TrailerError::Generic(format!("cannot get homedir")))?;

    // search paths for config files, in order of search preference.
    let search_paths = vec![
        format!("./.config.toml"),
        format!("{}/.config.toml", home_path.display()),
        format!("{}/.crypto/.config.toml", home_path.display()),
    ];

    for path in search_paths.clone() {
        if file_exists(&path) {
            // info!("loading config from {}", path);
            return Ok(toml::from_str(&str_from_file_path(&path)?)?);
        }
    }

    Err(Box::new(TrailerError::ConfigError(format!(
        "could not find a config file in the following locations: {:?}",
        search_paths
    ))))
}
