use serde::Serialize;
use sqlx::prelude::*;
use uuid::Uuid;

#[derive(Debug, FromRow, PartialEq, Clone, Serialize)]
pub struct Setting {
    pub id: Uuid,
    pub key: String,
    pub value: String,
}
