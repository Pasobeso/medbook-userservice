use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use utoipa_axum::router::OpenApiRouter;

use crate::{
    application::usecases::users::UsersUseCase,
    domain::{
        repositories::users::UsersRepository,
        value_objects::users_model::{
            FindUserByIdResponseModel, RegisterUserModel, RegisterUserResponseModel,
        },
    },
    infrastructure::{
        axum_http::api_response::ApiResponse,
        postgres::{postgres_connection::PgPoolSquad, repositories::users::UsersPostgres},
    },
};

#[deprecated]
pub fn routes(db_pool: PgPoolSquad) -> Router {
    let users_repository = UsersPostgres::new(db_pool);
    let users_use_case = UsersUseCase::new(Arc::new(users_repository));

    Router::new()
        .route("/", post(register))
        .route("/:user_id", get(find_by_id))
        .with_state(Arc::new(users_use_case))
}

/// Defines routes with OpenAPI specs. Should be used over `routes()` where possible.
pub fn routes_with_openapi(db_pool: PgPoolSquad) -> OpenApiRouter {
    let users_repository = UsersPostgres::new(db_pool);
    let users_use_case = UsersUseCase::new(Arc::new(users_repository));

    OpenApiRouter::new().nest(
        "/users",
        OpenApiRouter::new()
            .routes(utoipa_axum::routes!(register))
            .routes(utoipa_axum::routes!(find_by_id))
            .with_state(Arc::new(users_use_case)),
    )
}

/// Registers a new user (patient or doctor) in the system.
#[utoipa::path(
    post,
    path = "/",
    tags = ["Users"],
    request_body = RegisterUserModel,
    responses(
        (status = 201, description = "User registered successfully", body = ApiResponse<RegisterUserResponseModel>)
    )
)]
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

/// Registers a new user (patient or doctor) in the system.
#[utoipa::path(
    get,
    path = "/:user_id",
    tags = ["Users"],
    responses(
        (status = 201, description = "Find user by id successfully", body = ApiResponse<FindUserByIdResponseModel>)
    )
)]
pub async fn find_by_id<T>(
    State(users_use_case): State<Arc<UsersUseCase<T>>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    match users_use_case.find_by_id(user_id).await {
        Ok(user_entity) => {
            let data = FindUserByIdResponseModel {
                id: user_entity.id,
                citizen_id: user_entity.citizen_id.clone(),
                first_name: user_entity.first_name.clone(),
                last_name: user_entity.last_name.clone(),
                phone_number: user_entity.phone_number.clone(),
                role: user_entity.role.clone(),
                created_at: user_entity.created_at,
                updated_at: user_entity.updated_at,
                deleted_at: user_entity.deleted_at,
            };
            (
                StatusCode::CREATED,
                Json(ApiResponse {
                    data: Some(data),
                    message: Some(format!("Get user id: {} successfully", user_id)),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<FindUserByIdResponseModel> {
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}
