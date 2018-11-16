#![allow(dead_code)]
#![allow(unused_variables)]

use toml;
use ::error::*;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub exchange: BTreeMap<String, APIConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct APIConfig {
    pub api_key: String,
    pub secret_key: String,
    pub positions: Option<Vec<String>>,
    pub watch: Option<Vec<String>>,
}

impl From<::std::io::Error> for TrailerError {
    fn from(error: ::std::io::Error) -> Self {
        use std::error::Error;

        TrailerError {
            error_type: TrailerErrorType::ConfigError,
            message: format!("cannot read .config.toml: {}", error.description()),
        }
    }
}

impl From<::std::string::FromUtf8Error> for TrailerError {
    fn from(error: ::std::string::FromUtf8Error) -> Self {
        use std::error::Error;

        TrailerError {
            error_type: TrailerErrorType::ConfigError,
            message: format!("cannot parse .config.toml to UTF8: {}", error.description()),
        }
    }
}

impl From<::toml::de::Error> for TrailerError {
    fn from(error: ::toml::de::Error) -> Self {
        use std::error::Error;

        TrailerError {
            error_type: TrailerErrorType::ConfigError,
            message: format!("cannot read .config.toml: {}", error.description()),
        }
    }
}

pub fn read(debug: bool) -> Result<Config, TrailerError> {
    pub fn file_exists(path: &str) -> bool {
        use std::fs;

        match fs::metadata(path) {
            Ok(p) => p.is_file(),
            Err(_) => false,
        }
    }

    fn str_from_file_path(path: &str) -> Result<String, TrailerError> {
        use std::io::prelude::*;

        let mut handle = ::std::fs::File::open(path)?;
        let mut bytebuffer = Vec::new();

        handle.read_to_end(&mut bytebuffer)?;

        return Ok(String::from_utf8(bytebuffer)?)
    }

    let home_path = ::std::env::home_dir().ok_or(TrailerError::generic("cannot get homedir"))?;

    // search paths for config files, in order of search preference.
    let search_paths = vec![
        format!("./.config.toml"),
        format!("{}/.config.toml", home_path.display()),
        format!("{}/.crypto/.config.toml", home_path.display()),
    ];

    for path in search_paths.clone() {
        if file_exists(&path) {
            if debug { println!("loading config from {}", path) };
            return Ok(toml::from_str(&str_from_file_path(&path)?)?);
        }
    };

    Err(TrailerError::generic(&format!("could not find a config file in the following locations: {:?}", search_paths)))
}