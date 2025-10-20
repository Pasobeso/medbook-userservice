use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
use utoipa::openapi::InfoBuilder;

use crate::{
    config::{config_loader, config_model::DotEnvyConfig, stage::Stage},
    infrastructure::{
        axum_http::{routers, swagger},
        postgres::postgres_connection::PgPoolSquad,
    },
};

use super::default_routers;

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: PgPoolSquad) -> Result<()> {
    let routes = routers::authentication::routes_with_openapi(db_pool.clone())
        .merge(routers::users::routes_with_openapi(db_pool.clone()));

    let mut openapi = routes.get_openapi().clone();
    openapi.info = InfoBuilder::new()
        .title("MedBook UserService API")
        .version("1.0.0")
        .build();
    let swagger_ui = swagger::create_swagger_ui(openapi)?;

    let mut app = Router::new()
        .fallback(default_routers::not_found)
        // .nest("/users", routers::users::routes(db_pool.clone()))
        // .nest("/authentication", routers::authentication::routes(db_pool))
        .merge(routes)
        .merge(swagger_ui)
        .route("/health-check", get(default_routers::health_check))
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout,
        )))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(TraceLayer::new_for_http());

    let development_cors_layer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true)
        .allow_origin(
            config
                .frontend
                .development_url
                .parse::<HeaderValue>()
                .unwrap(),
        );

    let production_cors_layer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true)
        .allow_origin(
            config
                .frontend
                .production_url
                .parse::<HeaderValue>()
                .unwrap(),
        );

    let local_cors_layer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers(Any)
        .allow_origin(Any);

    match config_loader::get_stage() {
        Stage::Production => {
            app = app.layer(production_cors_layer);
        }
        Stage::Development => {
            app = app.layer(development_cors_layer);
        }
        Stage::Local => {
            app = app.layer(local_cors_layer);
        }
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await?;

    info!("Server is running on port {}", config.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
    };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C signal"),
        _ = terminate => info!("Received terminate signal")
    }
}
