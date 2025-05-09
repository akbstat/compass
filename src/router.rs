use crate::{Config, Result};
use ::vestige::VestigeUsecase;
use apidoc::ApiDoc;
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

mod apidoc;
mod vestige;

pub async fn init_router(config: &Config) -> Result<Router> {
    let pool_vestige = Arc::new(PgPool::connect(&config.database_url).await?);
    let vestige_uc = VestigeUsecase::new(pool_vestige);
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/vestige", vestige::router(vestige_uc))
        .split_for_parts();
    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));
    Ok(router)
}
