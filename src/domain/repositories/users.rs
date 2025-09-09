use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::users::{RegisterUserEntity, UserEntity};

#[async_trait]
#[automock]
pub trait UsersRepository {
    async fn register(&self, register_user_entity: RegisterUserEntity) -> Result<i32>; 
    async fn find_by_id(&self, id: i32) -> Result<UserEntity>;
}