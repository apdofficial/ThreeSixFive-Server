use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerDocument {
    /// Document Id
    pub _id: ObjectId,
    /// customer name
    pub name: String,
    /// created_at
    pub createdAt: DateTime,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Customer {
    /// Document Id
    pub _id: String,
    /// customer name
    pub name: String,
    /// created_at
    pub createdAt: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CustomerInput {
    /// customer name
    pub name: String,
}
