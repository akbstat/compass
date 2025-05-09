use super::apidoc::VESTIGE_TAG;
use crate::errors::Result;
use apis::vestige::{
    History, ListHistoriesReply, ListHistoriesRequest, RemoveHistoriesReply, RemoveHistoriesRequest, SaveHistoryReply, SaveHistoryRequest
};
use axum::{
    extract::{Query, State}, Json
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
    params(ListHistoriesRequest), 
    tag = VESTIGE_TAG,
)]
async fn list_histories(
    State(uc): State<VestigeUsecase>,
    Query(request): Query<ListHistoriesRequest>,
) -> Result<Json<ListHistoriesReply>> {
    let user = request.user;
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
    State(uc): State<VestigeUsecase>,
    Json(request): Json<SaveHistoryRequest>,
) -> Result<Json<SaveHistoryReply>> {
    let SaveHistoryRequest {
        user,
        product,
        trial,
        purpose,
    } = request;
    let data = uc
        .save_history(
            &user,
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