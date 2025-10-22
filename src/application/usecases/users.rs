use std::sync::Arc;

use anyhow::Result;

use crate::{
    domain::{
        entities::users::UserEntity, repositories::users::UsersRepository,
        value_objects::users_model::RegisterUserModel,
    },
    infrastructure::argon2_hashing,
};

pub struct UsersUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    users_repository: Arc<T>,
}

impl<T> UsersUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    pub fn new(users_repository: Arc<T>) -> Self {
        Self { users_repository }
    }

    pub async fn register(&self, mut register_user_model: RegisterUserModel) -> Result<i32> {
        let hashed_password = argon2_hashing::hash(register_user_model.password.clone())?;

        register_user_model.password = hashed_password;

        let register_entity = register_user_model.to_entity();

        let user_id = self.users_repository.register(register_entity).await?;

        Ok(user_id)
    }

    pub async fn find_by_id(&self, user_id: i32) -> Result<UserEntity> {
        let user_entity = self.users_repository.find_by_id(user_id).await?;

        Ok(user_entity)
    }
}
