use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    source: &'static str,
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self {
            source: "min_auth::error::Error",
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.message, self.source)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self {
            source: "min_auth::error::Error",
            message: value,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            source: "std::io::Error",
            message: value.to_string(),
        }
    }
}

impl From<serde::de::value::Error> for Error {
    fn from(value: serde::de::value::Error) -> Self {
        Self {
            source: "serde::de::value::Error",
            message: value.to_string(),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self {
            source: "toml::de::Error",
            message: value.to_string(),
        }
    }
}

impl From<getopts::Fail> for Error {
    fn from(value: getopts::Fail) -> Self {
        Self {
            source: "getopts::Fail",
            message: value.to_string(),
        }
    }
}

impl From<redis::RedisError> for Error {
    fn from(value: redis::RedisError) -> Self {
        Self {
            source: "redis::RedisError",
            message: value.to_string(),
        }
    }
}

impl From<mysql_async::Error> for Error {
    fn from(value: mysql_async::Error) -> Self {
        Self {
            source: "mysql_async::Error",
            message: value.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self {
            source: "std::num::ParseIntError",
            message: value.to_string(),
        }
    }
}