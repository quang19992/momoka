use scylla::transport::errors::NewSessionError;
use std::convert;

#[derive(Clone, Debug)]
pub enum DatabaseError {
    ScyllaError(NewSessionError),
    R2d2Error(String),
}

impl convert::From<NewSessionError> for DatabaseError {
    fn from(err: NewSessionError) -> Self {
        DatabaseError::ScyllaError(err)
    }
}

impl convert::From<r2d2::Error> for DatabaseError {
    fn from(err: r2d2::Error) -> Self {
        DatabaseError::R2d2Error(err.to_string())
    }
}
