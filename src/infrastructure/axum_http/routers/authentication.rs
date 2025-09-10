use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::post,
};

use axum_extra::extract::cookie::{Cookie, CookieJar};
use cookie::time::Duration;

use crate::{
    application::usecases::authentication::AuthenticationUseCase,
    config::{config_loader::get_stage, stage::Stage},
    domain::{
        repositories::users::UsersRepository,
        value_objects::authentication_model::LoginResponseModel,
    },
    infrastructure::{
        axum_http::api_response::ApiResponse,
        jwt_authentication::authentication_model::LoginModel,
        postgres::{postgres_connection::PgPoolSquad, repositories::users::UsersPostgres},
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let users_repository = UsersPostgres::new(Arc::clone(&db_pool));
    let authentication_use_case = AuthenticationUseCase::new(Arc::new(users_repository));

    Router::new()
        .route("/patients/login", post(patients_login))
        .route("/patients/refresh-token", post(patients_refresh_token))
        .route("/doctors/login", post(doctors_login))
        .route("/doctors/refresh-token", post(doctors_refresh_token))
        .with_state(Arc::new(authentication_use_case))
}

pub async fn patients_login<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    match authentication_use_case.patients_login(login_model).await {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                act_cookie = act_cookie.secure(true);
                rft_cookie = rft_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );
            (
                StatusCode::OK,
                headers,
                Json(ApiResponse::<LoginResponseModel> {
                    data: None,
                    message: Some("Login successfully".to_string()),
                }),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<LoginResponseModel> {
                data: None,
                message: Some(e.to_string()),
            }),
        )
            .into_response(),
    }
}

pub async fn patients_refresh_token<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    if let Some(rft) = jar.get("rft") {
        let refresh_token = rft.value().to_string();
        let response = match authentication_use_case
            .patients_refresh_token(refresh_token)
            .await
        {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    act_cookie = act_cookie.secure(true);
                    rft_cookie = rft_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                );

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                );

                (
                    StatusCode::OK,
                    headers,
                    Json(ApiResponse::<LoginResponseModel> {
                        data: None,
                        message: Some("Login successfully".to_string()),
                    }),
                )
                    .into_response()
            }
            Err(e) => (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<LoginResponseModel> {
                    data: None,
                    message: Some(e.to_string()),
                }),
            )
                .into_response(),
        };

        return response;
    }

    (StatusCode::BAD_REQUEST, "Refresh token not found").into_response()
}

pub async fn doctors_login<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    match authentication_use_case.doctors_login(login_model).await {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                act_cookie = act_cookie.secure(true);
                rft_cookie = rft_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );

            (
                StatusCode::OK,
                headers,
                Json(ApiResponse::<LoginResponseModel> {
                    data: None,
                    message: Some("Login successfully".to_string()),
                }),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<LoginResponseModel> {
                data: None,
                message: Some(e.to_string()),
            }),
        )
            .into_response(),
    }
}
pub async fn doctors_refresh_token<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    if let Some(rft) = jar.get("rft") {
        let refresh_token = rft.value().to_string();
        let response = match authentication_use_case
            .doctors_refresh_token(refresh_token)
            .await
        {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    act_cookie = act_cookie.secure(true);
                    rft_cookie = rft_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                );

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                );

                (
                    StatusCode::OK,
                    headers,
                    Json(ApiResponse::<LoginResponseModel> {
                        data: None,
                        message: Some("Login successfully".to_string()),
                    }),
                )
                    .into_response()
            }
            Err(e) => (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<LoginResponseModel> {
                    data: None,
                    message: Some(e.to_string()),
                }),
            )
                .into_response(),
        };

        return response;
    }

    (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse::<LoginResponseModel> {
            data: None,
            message: Some("Refresh token not found".to_string()),
        }),
    )
        .into_response()
}
