use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    domain::entities::users::UserEntity, infrastructure::jwt_authentication::jwt_model::Claims,
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginResponseModel {}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetMeResponseModel {
    pub claims: Claims,
    pub me: UserEntity,
}
