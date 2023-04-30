use crate::graphql::error::{ClientFault, GraphQLError};
use juniper::{GraphQLScalar, InputValue, ScalarValue, Value};
use serde::{Deserialize, Serialize};

#[derive(GraphQLScalar, Serialize, Deserialize)]
#[graphql(
    description = "An implementation for twitter-like's snowflake.",
    to_output_with = resolve,
    from_input_with = from_input_value,
    parse_token(String),
)]
pub struct Snowflake(u64);

/// Output snowflake as a `String` since graphql specs
///     had no concept of a 64bits integer.
fn resolve<S: ScalarValue>(v: &Snowflake) -> Value<S> {
    Value::from(v.0.to_string())
}

/// The input send from the client will be a `String`
///     we need to convert it back to a snowflake.
fn from_input_value<S: ScalarValue>(v: &InputValue<S>) -> Result<Snowflake, GraphQLError> {
    let snowflake_str = match v.as_string_value() {
        None => return Err(GraphQLError::ClientFault(ClientFault::MustNotEmpty())),
        Some(snowflake_str) => snowflake_str,
    };
    Ok(match snowflake_str.parse::<u64>() {
        Err(_) => {
            return Err(GraphQLError::ClientFault(ClientFault::InvalidInteger(
                snowflake_str.to_owned(),
            )))
        }
        Ok(snowflake) => Snowflake(snowflake),
    })
}
