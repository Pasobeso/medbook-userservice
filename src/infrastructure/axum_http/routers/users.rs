use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    application::usecases::users::UsersUseCase,
    domain::{
        repositories::users::UsersRepository,
        value_objects::users_model::{RegisterUserModel, RegisterUserResponseModel},
    },
    infrastructure::{
        axum_http::api_response::ApiResponse,
        postgres::{postgres_connection::PgPoolSquad, repositories::users::UsersPostgres},
    },
};

pub fn routes(db_pool: PgPoolSquad) -> Router {
    let users_repository = UsersPostgres::new(db_pool);
    let users_use_case = UsersUseCase::new(Arc::new(users_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(users_use_case))
}

pub async fn register<T>(
    State(users_use_case): State<Arc<UsersUseCase<T>>>,
    Json(register_user_model): Json<RegisterUserModel>,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    match users_use_case.register(register_user_model).await {
        Ok(user_id) => {
            let data = RegisterUserResponseModel {
                hospital_number: user_id,
            };
            (
                StatusCode::CREATED,
                Json(ApiResponse {
                    data: Some(data),
                    message: Some(format!("Register user id: {} successfully", user_id)),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<RegisterUserResponseModel> {
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}
