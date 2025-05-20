use utoipa::OpenApi;

pub const VESTIGE_TAG: &str = "vestige";
pub const SDTM_METADATA_TAG: &str = "sdtm/metadata";
pub const SDTM_RAWDATA_TAG: &str = "sdtm/rawdata";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = VESTIGE_TAG, description= "Vestige API endpoints")
    )
)]
pub struct ApiDoc;
