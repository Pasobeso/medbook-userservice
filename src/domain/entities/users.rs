use chrono::NaiveDateTime;
use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::infrastructure::postgres::schema::users;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = users)]
pub struct UserEntity {
    pub id: i32,
    pub citizen_id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub password: String,
    pub role: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct RegisterUserEntity {
    pub citizen_id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub password: String,
    pub role: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
