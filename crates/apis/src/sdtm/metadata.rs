use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, ToSchema)]
pub struct Language {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct CreateLanguageRequest {
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateLanguageReply {
    pub data: Language,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListLanguageReply {
    pub data: Vec<Language>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VariableType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTypeRequest {
    pub name: String,
    pub lang_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateTypeReply {
    pub data: VariableType,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VariableCore {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCoreRequest {
    pub name: String,
    pub lang_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateCoreReply {
    pub data: VariableCore,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VariableRole {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoleRequest {
    pub name: String,
    pub lang_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateRoleReply {
    pub data: VariableRole,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SdtmVersion {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateSdtmVersionRequest {
    pub name: String,
    pub lang_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateSdtmVersionReply {
    pub data: SdtmVersion,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListSdtmVersionRequest {
    pub lang_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListSdtmVersionReply {
    pub data: Vec<SdtmVersion>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SdtmDomain {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateSdtmDomainRequest {
    pub name: String,
    pub description: String,
    pub version_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateSdtmDomainReply {
    pub data: SdtmDomain,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListSdtmDomainsRequest {
    pub version_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListSdtmDomainsReply {
    pub data: Vec<SdtmDomain>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SdtmVariable {
    pub id: i32,
    pub name: String,
    pub label: String,
    pub variable_type: String,
    pub codelist: String,
    pub variable_core: String,
    pub variable_role: String,
    pub variable_order: i32,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateSdtmVariableRequest {
    pub name: String,
    pub label: String,
    pub domain_id: i32,
    pub variable_type_id: i32,
    pub codelist: String,
    pub variable_core_id: i32,
    pub variable_role_id: i32,
    pub variable_order: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateSdtmVariableReply {
    pub data: SdtmVariable,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListSdtmVariableRequest {
    pub domain_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListSdtmVariableReply {
    pub data: Vec<SdtmVariable>,
}
