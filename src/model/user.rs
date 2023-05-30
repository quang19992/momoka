use super::Snowflake;
use serde::{Serialize, Deserialize};
use juniper::GraphQLObject;

#[derive(GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Snowflake,
    pub name: String,
    pub avatar: Option<String>,
    pub email: String,
}
