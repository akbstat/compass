use crate::{errors::Result, router::apidoc::SDTM_RAWDATA_TAG};
use apis::sdtm::rawdata::{
    CreateFormReply, CreateFormRequest, CreateItemOptionReply, CreateItemOptionRequest,
    CreateItemReply, CreateItemRequest, CreateItemTypeReply, CreateItemTypeRequest,
    CreateItemUnitReply, CreateItemUnitRequest, CreateProjectVersionReply,
    CreateProjectVersionRequest, ListFormsReply, ListFormsRequest, ListItemTypesReply,
    ListItemsReply, ListItemsRequest, ListProjectVersionReply, ListProjectVersionRequest,
};
use axum::{
    extract::{Query, State},
    Json,
};
use sdtm::RawdataUsecase;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router(usecase: RawdataUsecase) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_project_versions, create_project_version))
        .routes(routes!(list_forms, create_form))
        .routes(routes!(list_items, create_item))
        .routes(routes!(list_item_types, create_item_type))
        .routes(routes!(create_item_option))
        .routes(routes!(create_item_unit))
        .with_state(usecase)
}

#[utoipa::path(
    get,
    path = "/version",
    responses((status = OK, body = ListProjectVersionReply)),
    params(ListProjectVersionRequest),
    tag = SDTM_RAWDATA_TAG
)]
async fn list_project_versions(
    State(uc): State<RawdataUsecase>,
    Query(request): Query<ListProjectVersionRequest>,
) -> Result<Json<ListProjectVersionReply>> {
    let (project_id, data) = uc.list_project_versions(&request).await?;
    Ok(Json(ListProjectVersionReply { project_id, data }))
}

#[utoipa::path(
    get,
    path = "/form",
    responses((status = OK, body = ListFormsReply)),
    params(ListFormsRequest),
    tag = SDTM_RAWDATA_TAG
)]
async fn list_forms(
    State(uc): State<RawdataUsecase>,
    Query(request): Query<ListFormsRequest>,
) -> Result<Json<ListFormsReply>> {
    let data = uc.list_forms(&request).await?;
    Ok(Json(ListFormsReply { data }))
}

#[utoipa::path(
    get,
    path = "/item",
    responses((status = OK, body = ListItemsReply)),
    params(ListItemsRequest),
    tag = SDTM_RAWDATA_TAG
)]
async fn list_items(
    State(uc): State<RawdataUsecase>,
    Query(request): Query<ListItemsRequest>,
) -> Result<Json<ListItemsReply>> {
    let data = uc.list_items(&request).await?;
    Ok(Json(ListItemsReply { data }))
}

#[utoipa::path(
    get,
    path = "/item/type",
    responses((status = OK, body = ListItemTypesReply)),
    tag = SDTM_RAWDATA_TAG
)]
async fn list_item_types(State(uc): State<RawdataUsecase>) -> Result<Json<ListItemTypesReply>> {
    let data = uc.list_item_types().await?;
    Ok(Json(ListItemTypesReply { data }))
}

#[utoipa::path(
    post,
    path = "/version",
    request_body = CreateProjectVersionRequest,
    responses((status = OK, body = CreateProjectVersionReply)),
    tag = SDTM_RAWDATA_TAG
)]
async fn create_project_version(
    State(uc): State<RawdataUsecase>,
    Json(request): Json<CreateProjectVersionRequest>,
) -> Result<Json<CreateProjectVersionReply>> {
    let data = uc.create_project_version(&request).await?;
    Ok(Json(CreateProjectVersionReply { data }))
}

#[utoipa::path(
    post,
    path = "/form",
    request_body = CreateFormRequest,
    responses((status = OK, body = CreateFormReply)),
    tag = SDTM_RAWDATA_TAG
)]
async fn create_form(
    State(uc): State<RawdataUsecase>,
    Json(request): Json<CreateFormRequest>,
) -> Result<Json<CreateFormReply>> {
    let data = uc.create_form(&request).await?;
    Ok(Json(CreateFormReply { data }))
}

#[utoipa::path(
    post,
    path = "/item/type",
    request_body = CreateItemTypeRequest,
    responses((status = OK, body = CreateItemTypeReply)),
    tag = SDTM_RAWDATA_TAG
)]
async fn create_item_type(
    State(uc): State<RawdataUsecase>,
    Json(request): Json<CreateItemTypeRequest>,
) -> Result<Json<CreateItemTypeReply>> {
    let data = uc.create_item_type(&request).await?;
    Ok(Json(CreateItemTypeReply { data }))
}

#[utoipa::path(
    post,
    path = "/item",
    request_body = CreateItemRequest,
    responses((status = OK, body = CreateItemReply)),
    tag = SDTM_RAWDATA_TAG
)]
async fn create_item(
    State(uc): State<RawdataUsecase>,
    Json(request): Json<CreateItemRequest>,
) -> Result<Json<CreateItemReply>> {
    let data = uc.create_item(&request).await?;
    Ok(Json(CreateItemReply { data }))
}

#[utoipa::path(
    post,
    path = "/item/option",
    request_body = CreateItemOptionRequest,
    responses((status = OK, body = CreateItemOptionReply)),
    tag = SDTM_RAWDATA_TAG
)]
async fn create_item_option(
    State(uc): State<RawdataUsecase>,
    Json(request): Json<CreateItemOptionRequest>,
) -> Result<Json<CreateItemOptionReply>> {
    let data = uc.create_item_option(&request).await?;
    Ok(Json(CreateItemOptionReply { data }))
}

#[utoipa::path(
    post,
    path = "/item/unit",
    request_body = CreateItemUnitRequest,
    responses((status = OK, body = CreateItemUnitReply)),
    tag = SDTM_RAWDATA_TAG
)]
async fn create_item_unit(
    State(uc): State<RawdataUsecase>,
    Json(request): Json<CreateItemUnitRequest>,
) -> Result<Json<CreateItemUnitReply>> {
    let data = uc.create_item_unit(&request).await?;
    Ok(Json(CreateItemUnitReply { data }))
}
