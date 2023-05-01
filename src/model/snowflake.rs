#![allow(dead_code)]

use crate::graphql::error::{ClientFault, GraphQLError};
use juniper::{GraphQLScalar, InputValue, ScalarValue, Value};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Give this project's snowflake an epoch time.
/// This is not needed, and usually because people want to dealt with Y2038,
///     but this snowflake implementation use 37 bits for timestamp, Y2038 won't be a problem.
///     I just want to give this a time mark.
/// The timestamp is 2023/04/08 00:00:00 JST
///     2023 - the year this project started
///     04/08 - a random day, picked randomly (obviously)
const MOMOKA_EPOCH: u64 = 1680879600000;

/// Number of bits used for storing timestamp in a snowflake.
const TIMESTAMP_SIZE: u8 = 37;

/// Number of bits used for storing cluster identifier.
const CLUSTER_SIZE: u8 = 12;

/// Number of bits used for an incremental value in snowflake generate job.
const INC_SIZE: u8 = 15;

#[derive(GraphQLScalar, Debug, Clone, Serialize, Deserialize)]
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

/// Get miliseconds since MOMOKA_EPOCH
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("(check your system's time)")
        .as_millis() as u64
        - MOMOKA_EPOCH
}

/// Get the current cluster id.
/// TODO - make this able to implement externally.
fn cluster_id() -> u16 {
    0
}

impl Snowflake {
    /// Create a snowflake from cluster id and inc id.
    /// This method assume, the input is already safe.
    fn with_cluster_and_inc_id(cluster: u16, inc: u16) -> Snowflake {
        Snowflake((now() << (64 - TIMESTAMP_SIZE)) | ((cluster as u64) << INC_SIZE) | inc as u64)
    }

    /// Generate snowflake in a bulk with a specific cluster id.
    pub fn generate_with_cluster_id(cluster: u16, count: u16) -> Result<Vec<Snowflake>, String> {
        if cluster >> CLUSTER_SIZE > 0 {
            return Err(format!(
                "cluster id must not be using more than {} bits.",
                CLUSTER_SIZE
            ));
        }
        if count >> INC_SIZE > 0 {
            return Err(format!(
                "requested amount of snowflake must not be a number that using more than {} bits.",
                INC_SIZE
            ));
        }
        Ok((0..count)
            .map(|inc| Self::with_cluster_and_inc_id(cluster, inc))
            .collect())
    }

    /// Generate snowflake in a bulk with default cluster id.
    pub fn generate(count: u16) -> Result<Vec<Snowflake>, String> {
        Self::generate_with_cluster_id(cluster_id(), count)
    }

    /// Generate a single snowflake with a specific cluster id.
    pub fn with_cluster_id(cluster: u16) -> Result<Snowflake, String> {
        if cluster >> CLUSTER_SIZE > 0 {
            return Err(format!(
                "cluster id must not be using more than {} bits.",
                CLUSTER_SIZE
            ));
        }
        Ok(Self::with_cluster_and_inc_id(cluster, 0))
    }

    /// Generate a single snowflake with the default cluster id.
    pub fn new() -> Snowflake {
        Self::with_cluster_id(cluster_id()).unwrap()
    }

    /// Get timestamp from current snowflake.
    pub fn timestamp(&self) -> u64 {
        (self.0 >> (64 - TIMESTAMP_SIZE)) + MOMOKA_EPOCH
    }

    /// Get cluster id from current snowflake.
    pub fn cluster_id(&self) -> u16 {
        const MAX_U64: u64 = !(0 as u64);
        const TIMESTAMP_MASK: u64 = !(MAX_U64 >> (TIMESTAMP_SIZE as u64));
        const MASK: u64 = MAX_U64 & !TIMESTAMP_MASK;
        ((self.0 & MASK) >> INC_SIZE) as u16
    }

    /// Get inc id from current snowflake.
    pub fn inc(&self) -> u16 {
        const MAX_U64: u64 = !(0 as u64);
        const MASK: u64 = !(MAX_U64 << INC_SIZE);
        (self.0 & MASK) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_with_new_method() {
        let snowflake = Snowflake::new();
        assert!(snowflake.timestamp() > 0);
        assert_eq!(snowflake.cluster_id(), 0);
        assert_eq!(snowflake.inc(), 0);
    }

    #[test]
    fn generate_with_a_custom_cluster_id() {
        const CLUSTER_ID: u16 = 2000;
        let snowflake = Snowflake::with_cluster_id(CLUSTER_ID).unwrap();
        assert_eq!(snowflake.cluster_id(), CLUSTER_ID);
        assert_eq!(snowflake.inc(), 0);
    }

    #[test]
    fn generate_with_an_invalid_cluster_id() {
        const CLUSTER_ID: u16 = 0xFFFF;
        let snowflake = Snowflake::with_cluster_id(CLUSTER_ID);
        assert!(matches!(snowflake, Err(_)));
    }

    #[test]
    fn generate_multiple_snowflake() {
        const CLUSTER_ID: u16 = 2000;
        const SIZE: u16 = 20;
        let snowflakes = Snowflake::generate_with_cluster_id(CLUSTER_ID, SIZE).unwrap();
        assert_eq!(
            snowflakes
                .iter()
                .map(|snowflake| snowflake.cluster_id())
                .collect::<Vec<u16>>(),
            (0..SIZE)
                .collect::<Vec<u16>>()
                .iter()
                .map(|_| CLUSTER_ID)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            snowflakes
                .iter()
                .map(|snowflake| snowflake.inc())
                .collect::<Vec<u16>>(),
            (0..SIZE).collect::<Vec<u16>>()
        );
    }

    #[test]
    fn generate_too_many_snowflake() {
        const SIZE: u16 = 0xFFFF;
        let snowflakes = Snowflake::generate(SIZE);
        assert!(matches!(snowflakes, Err(_)));
    }

    #[test]
    fn generate_with_an_invalid_cluster_id_2() {
        const CLUSTER_ID: u16 = 0xFFFF;
        const SIZE: u16 = 20;
        let snowflake = Snowflake::generate_with_cluster_id(CLUSTER_ID, SIZE);
        assert!(matches!(snowflake, Err(_)));
    }
}
