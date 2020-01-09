use diesel::result::{
    DatabaseErrorKind,
    Error as DBError,
};
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum MangapplizerError {
    #[error("There are too many manga results.")]
    TooManyMangas,

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Cannot find any content with the specified search data.")]
    EmptySearch,

    #[error("{0}")]
    DatabaseError(DBError),
    #[error("{0}")]
    RelationInsertionError(String),

    #[error("Unable to connect to Db!")]
    UnableToConnectToDb,

    #[error("Internal Server Error!")]
    InternalServerError,

    #[error("Cannot insert into database!: {0}")]
    InsertionError(String),
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
