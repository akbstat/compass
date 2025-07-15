use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

pub const VESTIGE_TAG: &str = "vestige";
pub const SDTM_METADATA_TAG: &str = "sdtm/metadata";
pub const SDTM_RAWDATA_TAG: &str = "sdtm/rawdata";
pub const SDTM_ANNOTATION_TAG: &str = "sdtm/annotation";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = VESTIGE_TAG, description= "Vestige API endpoints"),
        (name = SDTM_METADATA_TAG, description= "Sdtm Metadata API endpoints"),
        (name = SDTM_RAWDATA_TAG, description= "Sdtm Rawdata API endpoints"),
        (name = SDTM_ANNOTATION_TAG, description= "Sdtm Annotation API endpoints")
    ),
    modifiers(&SecurityAddon),
    security(
        ("api_key" = [])
    ),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Compass-User"))),
            )
        }
    }
}
