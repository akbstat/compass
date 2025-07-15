use super::apidoc::VESTIGE_TAG;
use crate::{errors::Result, middleware::user::CompassUser};
use apis::vestige::{
    History, ListHistoriesReply, RemoveHistoriesReply, RemoveHistoriesRequest, SaveHistoryReply, SaveHistoryRequest
};
use axum::{
    extract::State, Extension, Json
};
use utoipa_axum::{router::OpenApiRouter, routes};
use vestige::VestigeUsecase;

pub fn router(usecase: VestigeUsecase) -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_histories))
        .routes(routes!(save_history))
        .routes(routes!(remove_histories))
        .with_state(usecase)
}


/// List access history by user
#[utoipa::path(
    get, 
    path = "", 
    responses((status = OK, body = ListHistoriesReply)), 
    tag = VESTIGE_TAG,
)]
async fn list_histories(
    Extension(user): Extension<CompassUser>,
    State(uc): State<VestigeUsecase>,
) -> Result<Json<ListHistoriesReply>> {
    let user = user.0;
    let data = uc.list_histories(&user).await?;
    Ok(Json(ListHistoriesReply { data }))
}

/// save access history of user
#[utoipa::path(
    post, 
    path = "", 
    responses((status = OK, body = ListHistoriesReply)), 
    request_body = SaveHistoryRequest, 
    tag = VESTIGE_TAG,
)]
async fn save_history(
    Extension(user): Extension<CompassUser>,
    State(uc): State<VestigeUsecase>,
    Json(request): Json<SaveHistoryRequest>,
) -> Result<Json<SaveHistoryReply>> {
    let SaveHistoryRequest {
        product,
        trial,
        purpose,
    } = request;
    let data = uc
        .save_history(
            &user.0,
            &History {
                id: None,
                product,
                trial,
                purpose,
            },
        )
        .await?;
    Ok(Json(SaveHistoryReply { data }))
}

/// remove multiple access history by ids
#[utoipa::path(
    post, 
    path = "/remove", 
    responses((status = OK, body = RemoveHistoriesReply)), 
    request_body = RemoveHistoriesRequest, 
    tag = VESTIGE_TAG,
)]
async fn remove_histories(State(uc): State<VestigeUsecase>, Json(request): Json<RemoveHistoriesRequest>) -> Result<Json<RemoveHistoriesReply>> {
    let ids = request.ids;
    uc.remove_histories(&ids).await?;
    Ok(Json(RemoveHistoriesReply {  }))
}