#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
pub struct TrailerError {
    pub error_type: TrailerErrorType,
    pub message: String,
}

#[derive(Debug)]
pub enum TrailerErrorType {
    ImportError,
    APIError,
    MissingArgumentError(String),
    CommandError,
    ConfigError,
    Unsupported,
    Generic,
}

impl TrailerError {
    pub fn unsupported() -> Self {
        Self {
            error_type: TrailerErrorType::Unsupported,
            message: "This feature is not supported by the selected exchange.".into(),
        }
    }

    pub fn missing_argument(arg: &str) -> Self {
        Self {
            error_type: TrailerErrorType::MissingArgumentError(arg.into()),
            message: format!("Missing required argument: {}", arg),
        }
    }

    pub fn missing_config_keys(arg: &str) -> Self {
        Self {
            error_type: TrailerErrorType::ConfigError,
            message: format!("Missing required configuration keys for: {}", arg),
        }
    }

    pub fn missing_exchange_adaptor(arg: &str) -> Self {
        Self {
            error_type: TrailerErrorType::ConfigError,
            message: format!("Exchange adaptor specified in config.toml does not exist: {}", arg),
        }
    }

    pub fn generic(arg: &str) -> Self {
        Self {
            error_type: TrailerErrorType::Generic,
            message: arg.into(),
        }
    }
}

impl From<::std::num::ParseFloatError> for TrailerError {
    fn from(error: ::std::num::ParseFloatError) -> Self {
        TrailerError {
            error_type: TrailerErrorType::CommandError,
            message: "One or more provided parameters could not be converted to valid floats.".into(),
        }
    }
}

impl From<::std::num::ParseIntError> for TrailerError {
    fn from(_error: ::std::num::ParseIntError) -> Self {
        TrailerError {
            error_type: TrailerErrorType::CommandError,
            message: "One or more provided parameters could not be converted to valid integers.".into(),
        }
    }
}