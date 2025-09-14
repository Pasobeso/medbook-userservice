use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::users::UserEntity, infrastructure::jwt_authentication::jwt_model::Claims,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponseModel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMeResponseModel {
    pub claims: Claims,
    pub me: UserEntity,
}
