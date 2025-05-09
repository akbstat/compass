use utoipa::OpenApi;

pub const VESTIGE_TAG: &str = "vestige";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = VESTIGE_TAG, description= "Vestige API endpoints")
    )
)]
pub struct ApiDoc;
