use scylla::transport::errors::NewSessionError;
use std::convert;

#[derive(Clone, Debug)]
pub enum DatabaseError {
    ScyllaError(NewSessionError),
}

impl convert::From<NewSessionError> for DatabaseError {
    fn from(err: NewSessionError) -> Self {
        DatabaseError::ScyllaError(err)
    }
}
