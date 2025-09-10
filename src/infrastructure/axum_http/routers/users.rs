use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    application::usecases::users::UsersUseCase,
    domain::{repositories::users::UsersRepository, value_objects::user_model::RegisterUserModel},
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::users::UsersPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let users_repository = UsersPostgres::new(db_pool);
    let users_use_case = UsersUseCase::new(Arc::new(users_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(users_use_case))
}

pub async fn register<T>(
    State(adventurers_use_case): State<Arc<UsersUseCase<T>>>,
    Json(register_user_model): Json<RegisterUserModel>,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    match adventurers_use_case.register(register_user_model).await {
        Ok(adventurer_id) => (
            StatusCode::CREATED,
            format!("Register adventurer id: {} successfully", adventurer_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
