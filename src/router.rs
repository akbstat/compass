use crate::{middleware::user::compass_user_middleware, Config, Result};
use ::sdtm::{AnnotationUsecase, MetadataUsecase, RawdataUsecase};
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
mod sdtm;
mod vestige;

pub async fn init_router(config: &Config) -> Result<Router> {
    let pool_vestige = Arc::new(PgPool::connect(&config.database_url_vestige).await?);
    let pool_sdtm = Arc::new(PgPool::connect(&config.database_url_sdtm).await?);
    let vestige_uc = VestigeUsecase::new(pool_vestige);
    let sdtm_metadata_uc = MetadataUsecase::new(Arc::clone(&pool_sdtm));
    let sdtm_rawdata_uc = RawdataUsecase::new(Arc::clone(&pool_sdtm));
    let sdtm_annotation_uc = AnnotationUsecase::new(Arc::clone(&pool_sdtm));
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/vestige", vestige::router(vestige_uc))
        .nest(
            "/api/sdtm/metadata",
            sdtm::metadata::router(sdtm_metadata_uc),
        )
        .nest("/api/sdtm/rawdata", sdtm::rawdata::router(sdtm_rawdata_uc))
        .nest(
            "/api/sdtm/annotation",
            sdtm::annotation::router(sdtm_annotation_uc),
        )
        .layer(from_fn(compass_user_middleware))
        .split_for_parts();
    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api))
        .layer(TraceLayer::new_for_http());
    Ok(router)
}
