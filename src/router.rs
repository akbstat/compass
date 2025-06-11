use crate::{middleware::user::compass_user_middleware, Config, Result};
use ::vestige::VestigeUsecase;
use apidoc::ApiDoc;
use axum::{middleware::from_fn, Router};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
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
        .layer(TraceLayer::new_for_http())
        .layer(from_fn(compass_user_middleware))
        .split_for_parts();
    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));
    Ok(router)
}
