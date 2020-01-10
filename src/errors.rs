use actix_web::{
    error::ResponseError,
    HttpResponse,
};

use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DBError;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum MangapplizerError {
    #[error("There are too many manga results.")]
    TooManyMangas,

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Cannot find any content with the specified search data.")]
    EmptySearch,

    #[error("{0}")]
    RelationInsertionError(String),

    #[error("Unable to connect to Db!")]
    UnableToConnectToDb,

    #[error("Unable to insert data into Db!")]
    UnableToInsertIntoDb,

    #[error("Internal Server Error!")]
    InternalServerError,

    #[error("Cannot insert into database!: {0}")]
    InsertionError(String),
}

// impl ResponseError trait allows to convert our errors into http responses
// with appropriate data
impl ResponseError for MangapplizerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MangapplizerError::InternalServerError => {
                HttpResponse::InternalServerError()
                    .json("Internal Server Error, Please try later")
            }
            MangapplizerError::UnableToConnectToDb => {
                HttpResponse::InternalServerError()
                    .json("Unable to connect to DB, Please try later")
            }
            MangapplizerError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            }
            MangapplizerError::Unauthorized => {
                HttpResponse::Unauthorized().json("Unauthorized")
            }
            MangapplizerError::TooManyMangas => {
                HttpResponse::InternalServerError().json("Find too many mangas")
            }
            MangapplizerError::EmptySearch => {
                HttpResponse::InternalServerError().json("Cannot find mangas")
            }
            MangapplizerError::RelationInsertionError(ref msg) => {
                HttpResponse::InternalServerError().json(msg)
            }
            MangapplizerError::UnableToInsertIntoDb => {
                HttpResponse::InternalServerError()
                    .json("Unable to insert into the DB!")
            }
            MangapplizerError::InsertionError(ref msg) => {
                HttpResponse::InternalServerError().json(msg)
            }
        }
    }
}

impl From<DBError> for MangapplizerError {
    fn from(error: DBError) -> MangapplizerError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info
                        .details()
                        .unwrap_or_else(|| info.message())
                        .to_string();
                    return MangapplizerError::BadRequest(message);
                }
                MangapplizerError::InternalServerError
            }
            _ => MangapplizerError::InternalServerError,
        }
    }
}

pub type MangapplizerResult<T> =
    std::result::Result<T, crate::errors::MangapplizerError>;
