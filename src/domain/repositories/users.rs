use anyhow::Result;
use mockall::automock;

use crate::domain::{
    entities::users::{RegisterUserEntity, UserEntity},
    value_objects::roles::Roles,
};

#[async_trait::async_trait]
#[automock]
pub trait UsersRepository {
    async fn register(&self, register_user_entity: RegisterUserEntity) -> Result<i32>;
    async fn find_by_id(&self, id: i32) -> Result<UserEntity>;
    async fn remove_by_id(&self, id: i32) -> Result<()>;
    async fn add_role_to_user_by_id(&self, role: Roles, id: i32) -> Result<()>;
    async fn remove_role_from_user_by_id(&self, role: Roles, id: i32) -> Result<()>;
}
