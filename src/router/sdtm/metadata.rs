use crate::{errors::Result, router::apidoc::SDTM_METADATA_TAG};
use apis::sdtm::metadata::{CreateCoreReply, CreateCoreRequest, CreateLanguageReply, CreateLanguageRequest, CreateRoleReply, CreateRoleRequest, CreateSdtmDomainReply, CreateSdtmDomainRequest, CreateSdtmVariableReply, CreateSdtmVariableRequest, CreateSdtmVersionReply, CreateSdtmVersionRequest, CreateTypeReply, CreateTypeRequest, ListLanguageReply, ListSdtmDomainsReply, ListSdtmDomainsRequest, ListSdtmVariableReply, ListSdtmVariableRequest, ListSdtmVersionReply, ListSdtmVersionRequest};
use axum::{extract::{Query, State}, Json};
use sdtm::MetadataUsecase;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router(usecase: MetadataUsecase) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_languages, create_language))
        .routes(routes!(list_sdtm_versions, create_sdtm_version))
        .routes(routes!(list_sdtm_domains, create_sdtm_domain))
        .routes(routes!(list_sdtm_variables, create_sdtm_variable))
        .routes(routes!(create_type))
        .routes(routes!(create_core))
        .routes(routes!(create_role))
        .with_state(usecase)
}

/// List Languages
#[utoipa::path(
    get, 
    path = "/language", 
    responses((status = OK, body = ListLanguageReply)), 
    tag = SDTM_METADATA_TAG,
)]
async fn list_languages(State(uc): State<MetadataUsecase>) -> Result<Json<ListLanguageReply>> {
    let data = uc.list_languages().await?;
    Ok(Json(ListLanguageReply { data }))
}

/// List SDTM Versions
#[utoipa::path(
    get, 
    path = "/version", 
    responses((status = OK, body = ListLanguageReply)), 
    params(ListSdtmVersionRequest), 
    tag = SDTM_METADATA_TAG,
)]
async fn list_sdtm_versions(State(uc): State<MetadataUsecase>, Query(request): Query<ListSdtmVersionRequest>) -> Result<Json<ListSdtmVersionReply>> {
    let data = uc.list_sdtm_versions(request.lang_id).await?;
    Ok(Json(ListSdtmVersionReply { data }))
}

/// List SDTM Domains
#[utoipa::path(
    get, 
    path = "/domain", 
    responses((status = OK, body = ListSdtmDomainsReply)), 
    params(ListSdtmDomainsRequest), 
    tag = SDTM_METADATA_TAG,
)]
async fn list_sdtm_domains(State(uc): State<MetadataUsecase>, Query(request): Query<ListSdtmDomainsRequest>) -> Result<Json<ListSdtmDomainsReply>> {
    let data = uc.list_sdtm_domains(request.version_id).await?;
   Ok(Json(ListSdtmDomainsReply{ data }))
}

/// List SDTM Variable
#[utoipa::path(
    get, 
    path = "/variable", 
    responses((status = OK, body = ListSdtmVariableReply)), 
    params(ListSdtmVariableRequest), 
    tag = SDTM_METADATA_TAG,
)]
async fn list_sdtm_variables(State(uc): State<MetadataUsecase>, Query(request): Query<ListSdtmVariableRequest>) -> Result<Json<ListSdtmVariableReply>> {
    let data = uc.list_sdtm_variables(request.domain_id).await?;
    Ok(Json(ListSdtmVariableReply { data }))
}

/// Create Language
#[utoipa::path(
    post, 
    path = "/language", 
    responses((status = OK, body = CreateLanguageReply)), 
    params(CreateLanguageRequest), 
    tag = SDTM_METADATA_TAG,
)]
async fn create_language(State(uc): State<MetadataUsecase>, Json(request): Json<CreateLanguageRequest>) -> Result<Json<CreateLanguageReply>> {
    let data = uc.create_language(&request).await?;
    Ok(Json(CreateLanguageReply { data }))
}

/// Create Type
#[utoipa::path(
    post, 
    path = "/type", 
    responses((status = OK, body = CreateTypeReply)), 
    params(CreateTypeRequest), 
    tag = SDTM_METADATA_TAG,
)]
async fn create_type(State(uc): State<MetadataUsecase>, Json(request): Json<CreateTypeRequest>) -> Result<Json<CreateTypeReply>> {
    let data = uc.create_type(&request).await?;
    Ok(Json(CreateTypeReply { data }))
}

/// Create Core
#[utoipa::path(
    post, 
    path = "/core", 
    responses((status = OK, body = CreateCoreReply)), 
    params(CreateCoreRequest), 
    tag = SDTM_METADATA_TAG,
)]
async fn create_core(State(uc): State<MetadataUsecase>, Json(request): Json<CreateCoreRequest>) -> Result<Json<CreateCoreReply>> {
    let data = uc.create_core(&request).await?;
    Ok(Json(CreateCoreReply { data }))
}

/// Create Role
#[utoipa::path(
    post, 
    path = "/role", 
    responses((status = OK, body = CreateRoleReply)), 
    params(CreateRoleRequest), 
    tag = SDTM_METADATA_TAG,
)]
async fn create_role(State(uc): State<MetadataUsecase>, Json(request): Json<CreateRoleRequest>) -> Result<Json<CreateRoleReply>> {
    let data = uc.create_role(&request).await?;
    Ok(Json(CreateRoleReply { data }))
}

/// Create Version
#[utoipa::path(
    post, 
    path = "/version", 
    responses((status = OK, body = CreateSdtmVersionReply)), 
    params(CreateSdtmVersionRequest), 
    tag = SDTM_METADATA_TAG,
)]
async fn create_sdtm_version(State(uc): State<MetadataUsecase>, Json(request): Json<CreateSdtmVersionRequest>) -> Result<Json<CreateSdtmVersionReply>> {
    let data = uc.create_sdtm_version(&request).await?;
    Ok(Json(CreateSdtmVersionReply { data }))
}

/// Create Domain
#[utoipa::path(
    post,
    path = "/domain",
    responses((status = OK, body = CreateSdtmDomainReply)),
    params(CreateSdtmDomainRequest),
    tag = SDTM_METADATA_TAG,
)]
async fn create_sdtm_domain(State(uc): State<MetadataUsecase>, Json(request): Json<CreateSdtmDomainRequest>) -> Result<Json<CreateSdtmDomainReply>> {
    let data = uc.create_sdtm_domain(&request).await?;
    Ok(Json(CreateSdtmDomainReply { data }))
}

/// Create Variable
#[utoipa::path(
    post,
    path = "/variable",
    responses((status = OK, body = CreateSdtmVariableReply)),
    params(CreateSdtmVariableRequest),
    tag = SDTM_METADATA_TAG
)]
async fn create_sdtm_variable(State(uc): State<MetadataUsecase>, Json(request): Json<CreateSdtmVariableRequest>) -> Result<Json<CreateSdtmVariableReply>> {
    let data = uc.create_sdtm_variable(&request).await?;
    Ok(Json(CreateSdtmVariableReply{ data }))
}

