use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::{delete, get, post},
};

use axum_extra::extract::cookie::{Cookie, CookieJar};
use cookie::time::Duration;
use utoipa_axum::router::OpenApiRouter;

use crate::{
    application::usecases::authentication::AuthenticationUseCase,
    config::{
        config_loader::{self, get_stage},
        stage::Stage,
    },
    domain::{
        repositories::users::UsersRepository,
        value_objects::authentication_model::{GetMeResponseModel, LoginResponseModel},
    },
    infrastructure::{
        axum_http::api_response::ApiResponse,
        jwt_authentication::{self, authentication_model::LoginModel, jwt_model::Claims},
        postgres::{postgres_connection::PgPoolSquad, repositories::users::UsersPostgres},
    },
};

#[deprecated]
pub fn routes(db_pool: PgPoolSquad) -> Router {
    let users_repository = UsersPostgres::new(db_pool);
    let authentication_use_case = AuthenticationUseCase::new(Arc::new(users_repository));

    Router::new()
        .route("/patients/login", post(patients_login))
        .route("/patients/refresh-token", post(patients_refresh_token))
        .route("/doctors/login", post(doctors_login))
        .route("/doctors/refresh-token", post(doctors_refresh_token))
        .route("/me", get(get_me))
        .route("/logout", delete(logout))
        .with_state(Arc::new(authentication_use_case))
}

/// Defines routes with OpenAPI specs. Should be used over `routes()` where possible.
pub fn routes_with_openapi(db_pool: PgPoolSquad) -> OpenApiRouter {
    let users_repository = UsersPostgres::new(db_pool);
    let authentication_use_case = AuthenticationUseCase::new(Arc::new(users_repository));

    OpenApiRouter::new().nest(
        "/authentication",
        OpenApiRouter::new()
            .routes(utoipa_axum::routes!(patients_login))
            .routes(utoipa_axum::routes!(patients_refresh_token))
            .routes(utoipa_axum::routes!(doctors_login))
            .routes(utoipa_axum::routes!(doctors_refresh_token))
            .routes(utoipa_axum::routes!(get_me))
            .routes(utoipa_axum::routes!(logout))
            .with_state(Arc::new(authentication_use_case)),
    )
}

/// Logs in a patient and sets authentication cookies.
#[utoipa::path(
    post,
    path = "/patients/login",
    tags = ["Authentication"],
    request_body = LoginModel,
    responses(
        (status = 200, description = "Login successfully", body = ApiResponse<LoginResponseModel>)
    )
)]
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

/// Refreshes the patient's authentication tokens using the refresh cookie.
#[utoipa::path(
    post,
    path = "/patients/refresh-token",
    tags = ["Authentication"],
    responses(
        (status = 200, description = "Refreshed patient tokens successfully", body = ApiResponse<LoginResponseModel>)
    )
)]
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

/// Logs in a doctor and sets authentication cookies.
#[utoipa::path(
    post,
    path = "/doctors/login",
    tags = ["Authentication"],
    request_body = LoginModel,
    responses(
        (status = 200, description = "Login successfully", body = ApiResponse<LoginResponseModel>)
    )
)]
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

/// Refreshes the doctor's authentication tokens using the refresh cookie.
#[utoipa::path(
    post,
    path = "/doctors/refresh-token",
    tags = ["Authentication"],
    responses(
        (status = 200, description = "Refreshed doctor tokens successfully", body = ApiResponse<LoginResponseModel>)
    )
)]
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

/// Retrieves information about the currently authenticated user.
#[utoipa::path(
    get,
    path = "/me",
    tags = ["Authentication"],
    responses(
        (status = 200, description = "Fetched current user successfully", body = ApiResponse<GetMeResponseModel>)
    )
)]
pub async fn get_me<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    if let Some(act) = jar.get("act") {
        let act = act.value();
        let patients_secret = config_loader::get_patients_secret_env();
        let doctors_secret = config_loader::get_doctors_secret_env();

        if patients_secret.is_err() || doctors_secret.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<GetMeResponseModel> {
                    data: None,
                    message: Some("Internal server error".to_string()),
                }),
            );
        }

        let patient_claims =
            jwt_authentication::verify_token(patients_secret.unwrap().secret, act.into());

        let doctor_claims =
            jwt_authentication::verify_token(doctors_secret.unwrap().secret, act.into());

        let claims: Claims;

        if patient_claims.is_err() && doctor_claims.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<GetMeResponseModel> {
                    data: None,
                    message: Some("Internal server error".to_string()),
                }),
            );
        }

        if patient_claims.is_err() {
            claims = doctor_claims.unwrap();
        } else {
            claims = patient_claims.unwrap();
        }

        match claims.sub.parse::<i32>() {
            Ok(sub) => {
                let me = authentication_use_case.get_me(sub).await;
                match me {
                    Ok(me) => {
                        return (
                            StatusCode::OK,
                            Json(ApiResponse::<GetMeResponseModel> {
                                data: Some(GetMeResponseModel { claims, me }),
                                message: Some("Get me successfully".to_string()),
                            }),
                        );
                    }
                    Err(err) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiResponse::<GetMeResponseModel> {
                                data: None,
                                message: Some("Internal server error".to_string()),
                            }),
                        );
                    }
                }
            }
            Err(err) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<GetMeResponseModel> {
                        data: None,
                        message: Some("Internal server error".to_string()),
                    }),
                );
            }
        }
    }

    return (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse::<GetMeResponseModel> {
            data: None,
            message: Some("Access token not found".to_string()),
        }),
    );
}

/// Logs out the current user and clears authentication cookies.
#[utoipa::path(
    delete,
    path = "/logout",
    tags = ["Authentication"],
    responses(
        (status = 200, description = "Logged out successfully")
    )
)]
pub async fn logout<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    let mut act_cookie = Cookie::build(("act", ""))
        .path("/")
        .same_site(cookie::SameSite::Lax)
        .http_only(true)
        .max_age(Duration::seconds(0));

    let mut rft_cookie = Cookie::build(("rft", ""))
        .path("/")
        .same_site(cookie::SameSite::Lax)
        .http_only(true)
        .max_age(Duration::seconds(0));

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
        Json(ApiResponse::<()> {
            data: None,
            message: Some("Logged out successfully".to_string()),
        }),
    )
        .into_response()
}
