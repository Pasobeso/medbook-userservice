use anyhow::Result;
use axum::async_trait;
use diesel::{
    ExpressionMethods, QueryDsl,
    dsl::insert_into,
    sql_types::{Integer, Text},
};
use diesel_async::RunQueryDsl;

use crate::{
    domain::{
        entities::users::{RegisterUserEntity, UserEntity},
        repositories::users::UsersRepository,
        value_objects::roles::Roles,
    },
    infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::users},
};

pub struct UsersPostgres {
    db_pool: PgPoolSquad,
}

impl UsersPostgres {
    pub fn new(db_pool: PgPoolSquad) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UsersRepository for UsersPostgres {
    async fn register(&self, register_user_entity: RegisterUserEntity) -> Result<i32> {
        let mut conn = self.db_pool.get().await?;
        let result = insert_into(users::table)
            .values(register_user_entity)
            .returning(users::id)
            .get_result::<i32>(&mut conn)
            .await?;

        Ok(result)
    }
    async fn find_by_id(&self, id: i32) -> Result<UserEntity> {
        let mut conn = self.db_pool.get().await?;
        let result = users::table.find(id).get_result(&mut conn).await?;
        Ok(result)
    }

    async fn remove_by_id(&self, id: i32) -> Result<()> {
        let mut conn = self.db_pool.get().await?;
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .filter(users::deleted_at.is_null())
            .set((users::deleted_at.eq(chrono::Utc::now().naive_utc()),))
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    async fn add_role_to_user_by_id(&self, role: Roles, id: i32) -> Result<()> {
        let mut conn = self.db_pool.get().await?;
        let role_str = role.to_string();

        diesel::sql_query(
            r#"
            UPDATE users
            SET role = CASE
                WHEN NOT ($1 = ANY(role)) THEN array_append(role, $1)
                ELSE role
            END,
                updated_at = NOW()
            WHERE id = $2
        "#,
        )
        .bind::<Text, _>(role_str)
        .bind::<Integer, _>(id)
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    async fn remove_role_from_user_by_id(&self, role: Roles, id: i32) -> Result<()> {
        let mut conn = self.db_pool.get().await?;
        let role_str = role.to_string();

        // ลบทุก occurrence ของค่านั้นในอาเรย์
        diesel::sql_query(
            r#"
            UPDATE users
            SET role = array_remove(role, $1),
                updated_at = NOW()
            WHERE id = $2
        "#,
        )
        .bind::<Text, _>(role_str)
        .bind::<Integer, _>(id)
        .execute(&mut conn)
        .await?;

        Ok(())
    }
}
