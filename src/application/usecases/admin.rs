use std::sync::Arc;

use anyhow::Result;

use crate::domain::{repositories::users::UsersRepository, value_objects::roles::Roles};

pub struct AdminUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    users_repository: Arc<T>,
}

impl<T> AdminUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    pub fn new(users_repository: Arc<T>) -> Self {
        Self {
            users_repository,
        }
    }

    pub async fn assign_doctor_role(
        &self,
        executer_user_id: i32,
        target_user_id: i32
    ) -> Result<()> {
        let role = Roles::Doctor;
        self.users_repository.add_role_to_user_by_id(role, target_user_id).await
    }

    pub async fn remove_doctor_role(
        &self,
        executer_user_id: i32,
        target_user_id: i32
    ) -> Result<()> {
        let role = Roles::Doctor;
        self.users_repository.remove_role_from_user_by_id(role, target_user_id).await
    }

    pub async fn remove_user(
        &self,
        executer_user_id: i32,
        user_id: i32
    ) -> Result<()> {
        self.users_repository.remove_by_id(user_id).await
    }
    
}

