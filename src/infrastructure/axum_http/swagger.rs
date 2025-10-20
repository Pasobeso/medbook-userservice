use anyhow::Result;
use utoipa::openapi::OpenApi;
use utoipa_swagger_ui::Config;

use crate::config;

pub fn create_swagger_ui(api: OpenApi) -> Result<utoipa_swagger_ui::SwaggerUi> {
    let config = config::config_loader::load()?;
    Ok(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", api)
        .config(Config::from(format!(
            "{}api-docs/openapi.json",
            config.server.path_prefix
        ))))
}
