use crate::{Config, Result};
use ::sdtm::{MetadataUsecase, RawdataUsecase};
use ::vestige::VestigeUsecase;
use apidoc::ApiDoc;
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
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
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/vestige", vestige::router(vestige_uc))
        .nest(
            "/api/sdtm/metadata",
            sdtm::metadata::router(sdtm_metadata_uc),
        )
        .nest("/api/sdtm/rawdata", sdtm::rawdata::router(sdtm_rawdata_uc))
        .split_for_parts();
    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));
    Ok(router)
}
