use crate::{errors::Result, router::apidoc::SDTM_ANNOTATION_TAG};
use apis::sdtm::annotation::{
    CreateAnnotationReply, CreateAnnotationRequest, CreateAnnotationVersionReply,
    CreateAnnotationVersionRequest, CreateFormDomainReply, CreateFormDomainRequest,
    ListAnnotationByFormReply, ListAnnotationByFormRequest, ListAnnotationVersionReply,
    ListAnnotationVersionRequest, ListFormDomainReply, ListFormDomainRequest,
    ListVariableByFormReply, ListVariableByFormRequest, ModifyAnnotationVersionReply,
    ModifyAnnotationVersionRequest, UpdateAnnootationReply, UpdateAnnootationRequest,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use sdtm::AnnotationUsecase;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router(usecase: AnnotationUsecase) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(
            list_annotation_version,
            create_annotation_version,
            modify_annotation_version,
            remove_annotation_version
        ))
        .routes(routes!(
            list_form_domain,
            create_form_domain,
            remove_form_domain
        ))
        .routes(routes!(list_variable_by_form))
        .routes(routes!(
            create_annotation,
            list_annotation_by_form,
            update_annotation,
            remove_annotation,
        ))
        .with_state(usecase)
}

#[utoipa::path(
    get,
    path = "/version",
    responses((status = OK, body = ListAnnotationVersionReply)),
    params(ListAnnotationVersionRequest),
    tag = SDTM_ANNOTATION_TAG
)]
async fn list_annotation_version(
    State(uc): State<AnnotationUsecase>,
    Query(request): Query<ListAnnotationVersionRequest>,
) -> Result<Json<ListAnnotationVersionReply>> {
    let data = uc
        .list_annotation_versions(request.project_version_id)
        .await?;
    Ok(Json(ListAnnotationVersionReply { data }))
}

#[utoipa::path(
    post,
    path = "/version",
    responses((status = OK, body = CreateAnnotationVersionReply)),
    request_body = CreateAnnotationVersionRequest,
    tag = SDTM_ANNOTATION_TAG
)]
async fn create_annotation_version(
    State(uc): State<AnnotationUsecase>,
    Json(request): Json<CreateAnnotationVersionRequest>,
) -> Result<Json<CreateAnnotationVersionReply>> {
    let data = uc.create_annotation_version(&request).await?;
    Ok(Json(CreateAnnotationVersionReply { data }))
}

#[utoipa::path(
    put,
    path = "/version/{id}",
    responses((status = OK, body = ModifyAnnotationVersionReply)),
    request_body = ModifyAnnotationVersionRequest,
    tag = SDTM_ANNOTATION_TAG
)]
async fn modify_annotation_version(
    State(uc): State<AnnotationUsecase>,
    Path(id): Path<i32>,
    Json(request): Json<ModifyAnnotationVersionRequest>,
) -> Result<Json<ModifyAnnotationVersionReply>> {
    let data = uc.modify_annotation_version(id, &request).await?;
    Ok(Json(ModifyAnnotationVersionReply { data }))
}

#[utoipa::path(
    delete,
    path = "/version/{id}",
    responses((status = OK)),
    tag = SDTM_ANNOTATION_TAG
)]
async fn remove_annotation_version(
    State(uc): State<AnnotationUsecase>,
    Path(id): Path<i32>,
) -> Result<Json<()>> {
    uc.remove_annotation_version(id).await?;
    Ok(Json(()))
}

#[utoipa::path(
    post,
    path = "/domain",
    request_body = CreateFormDomainRequest,
    responses((status = OK, body = CreateFormDomainReply)),
    tag = SDTM_ANNOTATION_TAG
)]
async fn create_form_domain(
    State(uc): State<AnnotationUsecase>,
    Json(request): Json<CreateFormDomainRequest>,
) -> Result<Json<CreateFormDomainReply>> {
    let data = uc.create_form_domain(&request).await?;
    Ok(Json(CreateFormDomainReply { data }))
}

#[utoipa::path(
    get,
    path = "/domain",
    params(ListFormDomainRequest),
    responses((status = OK, body = ListFormDomainReply)),
    tag = SDTM_ANNOTATION_TAG
)]
async fn list_form_domain(
    State(uc): State<AnnotationUsecase>,
    Query(request): Query<ListFormDomainRequest>,
) -> Result<Json<ListFormDomainReply>> {
    let data = uc.list_form_domains(&request).await?;
    Ok(Json(ListFormDomainReply { data }))
}

#[utoipa::path(
    delete,
    path = "/domain/{id}",
    responses((status = OK)),
    tag = SDTM_ANNOTATION_TAG
)]
async fn remove_form_domain(
    State(uc): State<AnnotationUsecase>,
    Path(id): Path<i32>,
) -> Result<Json<()>> {
    uc.remove_form_domain(id).await?;
    Ok(Json(()))
}

#[utoipa::path(
    get,
    path = "/variable",
    params(ListVariableByFormRequest),
    responses((status = OK, body = ListVariableByFormReply)),
    tag = SDTM_ANNOTATION_TAG
)]
async fn list_variable_by_form(
    State(uc): State<AnnotationUsecase>,
    Query(request): Query<ListVariableByFormRequest>,
) -> Result<Json<ListVariableByFormReply>> {
    let data = uc.list_variable_by_form(&request).await?;
    Ok(Json(ListVariableByFormReply { data }))
}

#[utoipa::path(
    post,
    path = "/annotation",
    request_body = CreateAnnotationRequest,
    responses((status = OK, body = CreateAnnotationReply)),
    tag = SDTM_ANNOTATION_TAG
)]
async fn create_annotation(
    State(uc): State<AnnotationUsecase>,
    Json(request): Json<CreateAnnotationRequest>,
) -> Result<Json<CreateAnnotationReply>> {
    let data = uc.create_annotation(&request).await?;
    Ok(Json(CreateAnnotationReply { data }))
}

#[utoipa::path(
    get,
    path = "/annotation",
    params(ListAnnotationByFormRequest),
    responses((status = OK, body = ListAnnotationByFormReply)),
    tag = SDTM_ANNOTATION_TAG
)]
async fn list_annotation_by_form(
    State(uc): State<AnnotationUsecase>,
    Query(request): Query<ListAnnotationByFormRequest>,
) -> Result<Json<ListAnnotationByFormReply>> {
    let data = uc.list_annotation_by_form(&request).await?;
    Ok(Json(ListAnnotationByFormReply { data }))
}

#[utoipa::path(
    put,
    path = "/annotation/{id}",
    request_body = UpdateAnnootationRequest,
    responses((status = OK, body = UpdateAnnootationReply)),
    tag = SDTM_ANNOTATION_TAG
)]
async fn update_annotation(
    State(uc): State<AnnotationUsecase>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateAnnootationRequest>,
) -> Result<Json<UpdateAnnootationReply>> {
    let data = uc.update_annotation(id, &request).await?;
    Ok(Json(UpdateAnnootationReply { data }))
}

#[utoipa::path(
    delete,
    path = "/annotation/{id}",
    responses((status = OK)),
    tag = SDTM_ANNOTATION_TAG
)]
async fn remove_annotation(
    State(uc): State<AnnotationUsecase>,
    Path(id): Path<i32>,
) -> Result<Json<()>> {
    uc.remove_annotation(id).await?;
    Ok(Json(()))
}
