use core::fmt;
use diesel::result;

#[derive(Debug)]
pub enum MangapplizerError {
    Error(result::Error),
    TooManyMangas(),
    EmptySearch(),
    DbError(result::Error),
    RelationInsertionError(String),
}

impl From<result::Error> for MangapplizerError {
    fn from(error: result::Error) -> Self {
        MangapplizerError::DbError(error)
    }
}

impl std::error::Error for MangapplizerError {}

impl fmt::Display for MangapplizerError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            MangapplizerError::TooManyMangas() => {
                let msg = "There are too many manga results.";
                write!(f, "{}", msg)
            }
            MangapplizerError::EmptySearch() => {
                let msg =
                    "Cannot find any content with the specified search data.";
                write!(f, "{}", msg)
            }
            MangapplizerError::RelationInsertionError(error) => {
                write!(f, "Cannot inset {}.", error)
            }
            MangapplizerError::DbError(error) => write!(f, "{}", error),
            _ => {
                let msg = "Undefined error!";
                write!(f, "{}", msg)
            }
        }
    }
}
