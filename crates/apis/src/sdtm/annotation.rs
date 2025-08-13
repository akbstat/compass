use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationVersion {
    pub id: i32,
    pub project_version_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListAnnotationVersionRequest {
    pub project_version_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListAnnotationVersionReply {
    pub data: Vec<AnnotationVersion>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateAnnotationVersionRequest {
    pub project_version_id: i32,
    pub name: String,
    pub description: String,
    pub source_version_id: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateAnnotationVersionReply {
    pub data: AnnotationVersion,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModifyAnnotationVersionRequest {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ModifyAnnotationVersionReply {
    pub data: AnnotationVersion,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FormDomain {
    pub id: i32,
    pub annotation_version_id: i32,
    pub form_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateFormDomainRequest {
    pub annotation_version_id: i32,
    pub form_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateFormDomainReply {
    pub data: FormDomain,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListFormDomainRequest {
    pub annotation_version_id: i32,
    pub form_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListFormDomainReply {
    pub data: Vec<FormDomain>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FormVariable {
    pub id: i32,
    pub domain_id: i32,
    pub name: String,
    pub supp: bool,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListVariableByFormRequest {
    pub annotation_version_id: i32,
    pub form_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListVariableByFormReply {
    pub data: Vec<FormVariable>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    pub id: i32,
    pub annotation_version_id: i32,
    pub form_id: i32,
    pub variable_id: i32,
    pub source_id: i32,
    pub kind: AnnotationKind,
    pub annotation_display: String,
    pub assign: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AnnotationInForm {
    pub form: Vec<Annotation>,
    pub item: Vec<Annotation>,
    pub value: Vec<Annotation>,
    pub unit: Vec<Annotation>,
    pub option: Vec<Annotation>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListAnnotationByFormRequest {
    pub form_id: i32,
    pub annotation_version_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListAnnotationByFormReply {
    pub data: AnnotationInForm,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateAnnotationRequest {
    pub annotation_version_id: i32,
    pub form_id: i32,
    pub variable: Option<VariableBinding>,
    pub location: AnnotationLocation,
    pub annotation_display: String,
    pub assign: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateAnnotationReply {
    pub data: Annotation,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationLocation {
    pub source_id: i32,
    pub kind: AnnotationKind,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAnnootationRequest {
    // None stands for variable not change, Some(None) Stands for variable change to None
    pub variable: Option<VariableBinding>,
    pub annotation_display: String,
    pub assign: bool,
    pub not_submit: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UpdateAnnootationReply {
    pub data: Annotation,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VariableBinding {
    pub domain_id: i32,
    pub variable_name: String,
    pub supp: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum AnnotationKind {
    Form,
    Item,
    Value,
    Unit,
    Option,
    Unknown,
}
