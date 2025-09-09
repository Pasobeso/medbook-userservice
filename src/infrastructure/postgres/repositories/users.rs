use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::{dsl::insert_into, prelude::*};

use crate::{
    domain::{entities::users::{RegisterUserEntity, UserEntity}, repositories::users::UsersRepository},
    infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::users},
};

pub struct UserPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl UserPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UsersRepository for UserPostgres {
    async fn register(&self, register_user_entity: RegisterUserEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(users::table)
            .values(register_user_entity)
            .returning(users::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }
    async fn find_by_id(&self, id: i32) -> Result<UserEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = users::table
            .filter(users::id.eq(id))
            .select(UserEntity::as_select())
            .first::<UserEntity>(&mut conn)?;

        Ok(result)
    }
}
