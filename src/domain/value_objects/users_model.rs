use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::{entities::users::RegisterUserEntity, value_objects::roles::Roles};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterUserModel {
    pub citizen_id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub password: String,
}

impl RegisterUserModel {
    pub fn to_entity(&self) -> RegisterUserEntity {
        RegisterUserEntity {
            citizen_id: self.citizen_id.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            phone_number: self.phone_number.clone(),
            password: self.password.clone(),
            role: vec![Roles::Patient.to_string()],
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            deleted_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterUserResponseModel {
    pub hospital_number: i32,
}
