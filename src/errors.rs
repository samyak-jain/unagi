use std::sync::{PoisonError, RwLockWriteGuard};

use config::{Config, ConfigError};
use rocket::response::Responder;
use rocket::{http::Status, response::NamedFile};
use rocket_contrib::json::JsonValue;

pub enum ApiError {
    DieselError(diesel::result::Error),
    ConfigError(Option<ConfigError>),
    IOError(std::io::Error),
}

pub type ApiResponse = Result<JsonValue, ApiError>;
pub type FileResponse = Result<NamedFile, ApiError>;

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

impl From<ConfigError> for ApiError {
    fn from(error: ConfigError) -> Self {
        ApiError::ConfigError(Some(error))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> ApiError {
        ApiError::IOError(error)
    }
}
