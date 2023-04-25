use mysql::Error as MysqlError;
use scylla::{
    cql_to_rust::FromRowError as ScyllaFromRowError,
    transport::errors::{NewSessionError, QueryError as ScyllaQueryError},
};
use std::{convert, sync::Arc};

#[derive(Clone, Debug)]
pub enum DatabaseError {
    ScyllaError(NewSessionError),
    ScyllaQueryError(ScyllaQueryError),
    ScyllaFromRowError(ScyllaFromRowError),
    MysqlError(Arc<MysqlError>),
    R2d2Error(String),
    Other(String),
}

impl convert::From<NewSessionError> for DatabaseError {
    fn from(err: NewSessionError) -> Self {
        DatabaseError::ScyllaError(err)
    }
}

impl convert::From<ScyllaQueryError> for DatabaseError {
    fn from(err: ScyllaQueryError) -> Self {
        DatabaseError::ScyllaQueryError(err)
    }
}

impl convert::From<ScyllaFromRowError> for DatabaseError {
    fn from(err: ScyllaFromRowError) -> Self {
        DatabaseError::ScyllaFromRowError(err)
    }
}

impl convert::From<MysqlError> for DatabaseError {
    fn from(err: MysqlError) -> Self {
        DatabaseError::MysqlError(Arc::new(err))
    }
}

impl convert::From<r2d2::Error> for DatabaseError {
    fn from(err: r2d2::Error) -> Self {
        DatabaseError::R2d2Error(err.to_string())
    }
}
