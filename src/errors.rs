use std::{
    collections::HashMap,
    fmt,
    sync::Arc,
    sync::{MutexGuard, PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

use config::{Config, ConfigError};
use rocket::response::{status, Responder};
use rocket::{http::Status, response::NamedFile};
use rocket_contrib::json::JsonValue;
use shared_child::SharedChild;

pub enum ApiError {
    DieselError(diesel::result::Error),
    ConfigError(Option<ConfigError>),
    IOError(std::io::Error),
    TranscodingError(TranscodingError),
    NotFoundError(status::NotFound<&'static str>),
    MutexError,
}

pub type ApiResponse = Result<JsonValue, ApiError>;
pub type FileResponse = Result<NamedFile, ApiError>;

#[derive(Debug)]
pub struct TranscodingError {
    pub error: String,
}

impl Responder<'_, 'static> for ApiError {
    fn respond_to(self, request: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        let message = match self {
            ApiError::DieselError(error) => {
                if error == diesel::result::Error::NotFound {
                    return Err(Status::NotFound);
                }
                String::from(error.to_string())
            }
            ApiError::ConfigError(error) => match error {
                Some(err) => String::from(err.to_string()),
                None => String::from("Error in setting configuration"),
            },
            ApiError::IOError(error) => String::from(error.to_string()),
            ApiError::TranscodingError(error) => String::from(error.to_string()),
            ApiError::MutexError => String::from("Mutex was not able to lock properly"),
            ApiError::NotFoundError(error) => String::from(error.0),
        };

        error!("{}", message);

        json!({
            "status": "error"
        })
        .respond_to(request)
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(error: diesel::result::Error) -> ApiError {
        ApiError::DieselError(error)
    }
}

impl<'a> From<PoisonError<RwLockWriteGuard<'a, Config>>> for ApiError {
    fn from(_: PoisonError<RwLockWriteGuard<'a, Config>>) -> Self {
        ApiError::ConfigError(None)
    }
}

impl<'a> From<PoisonError<RwLockReadGuard<'a, Config>>> for ApiError {
    fn from(_: PoisonError<RwLockReadGuard<'a, Config>>) -> Self {
        ApiError::ConfigError(None)
    }
}

impl<'a> From<PoisonError<MutexGuard<'a, HashMap<String, Arc<SharedChild>>>>> for ApiError {
    fn from(_: PoisonError<MutexGuard<'a, HashMap<String, Arc<SharedChild>>>>) -> Self {
        ApiError::MutexError
    }
}

impl From<ConfigError> for ApiError {
    fn from(error: ConfigError) -> Self {
        ApiError::ConfigError(Some(error))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> Self {
        ApiError::IOError(error)
    }
}

impl From<TranscodingError> for ApiError {
    fn from(error: TranscodingError) -> Self {
        ApiError::TranscodingError(error)
    }
}

impl From<status::NotFound<&'static str>> for ApiError {
    fn from(error: status::NotFound<&'static str>) -> Self {
        ApiError::NotFoundError(error)
    }
}

impl fmt::Display for TranscodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error in transcoding")
    }
}

impl From<std::io::Error> for TranscodingError {
    fn from(error: std::io::Error) -> Self {
        TranscodingError {
            error: error.to_string(),
        }
    }
}
