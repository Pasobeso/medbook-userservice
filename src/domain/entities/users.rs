use chrono::NaiveDateTime;
use diesel::{prelude::{Identifiable, Insertable, Queryable}, Selectable};

use crate::{domain::value_objects::roles::Role, infrastructure::postgres::schema::users};

#[derive(Debug,Clone,Identifiable,Selectable,Queryable)]
#[diesel(table_name = users)]
pub struct UserEntity {
    pub id: i32,
    pub citizen_id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub password_hash: String,
    pub role: Vec<Role>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>
}

#[derive(Debug,Clone,Insertable,Queryable)]
#[diesel(table_name = users)]
pub struct RegisterUserEntity {
    pub citizen_id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub password_hash: String,
    pub role: Vec<Role>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>
}
