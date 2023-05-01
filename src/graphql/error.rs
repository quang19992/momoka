use juniper::{graphql_value, FieldError, IntoFieldError, ScalarValue};

#[derive(Debug, Clone)]
pub enum GraphQLError {
    /// When the error is made by the user (invalid input, unauthorized, etc...)
    ClientFault(ClientFault),
}

#[derive(Debug, Clone)]
pub enum ClientFault {
    /// When the field is not optional, but the client don't provide any input.
    MustNotEmpty(),

    /// The input is not a valid integer, return back what causes.
    InvalidInteger(String),
}

impl GraphQLError {
    fn name(&self) -> String {
        match self {
            Self::ClientFault(err) => format!("client fault - {}", err.name()),
        }
    }
}

impl<S: ScalarValue> IntoFieldError<S> for GraphQLError {
    fn into_field_error(self) -> FieldError<S> {
        FieldError::new(
            self.name(),
            graphql_value!({
                "message": format!("{:?}", self),
            }),
        )
    }
}

impl ClientFault {
    fn name(&self) -> String {
        match self {
            Self::MustNotEmpty() => "must not empty".to_owned(),
            Self::InvalidInteger(_) => "invalid integer".to_owned(),
        }
    }
}
