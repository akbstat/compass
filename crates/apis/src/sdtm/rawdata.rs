use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub struct CreateProjectRequest {
    pub name: String,
}

pub struct Project {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectVersionRequest {
    pub project_id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateProjectVersionReply {
    pub data: ProjectVersion,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectVersion {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListProjectVersionRequest {
    pub product: String,
    pub trial: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListProjectVersionReply {
    pub project_id: i32,
    pub data: Vec<ProjectVersion>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub form_order: i32,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateFormRequest {
    pub version_id: i32,
    pub name: String,
    pub description: String,
    pub form_order: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateFormReply {
    pub data: Form,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListFormsRequest {
    pub version_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListFormsReply {
    pub data: Vec<Form>,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct ItemType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct CreateItemTypeRequest {
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateItemTypeReply {
    pub data: ItemType,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub label: String,
    pub item_type_id: i32,
    pub item_order: i32,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetail {
    pub id: i32,
    pub name: String,
    pub label: String,
    pub item_type: Option<ItemType>,
    pub item_option: Option<Vec<ItemOption>>,
    pub item_unit: Option<Vec<ItemUnit>>,
    pub item_order: i32,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListItemsRequest {
    pub form_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListItemsReply {
    pub data: Vec<ItemDetail>,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemRequest {
    pub form_id: i32,
    pub name: String,
    pub label: String,
    pub item_type_id: i32,
    pub item_order: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateItemReply {
    pub data: Item,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemOption {
    pub id: i32,
    pub option_value: String,
    pub option_display: String,
    pub option_order: i32,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemOptionRequest {
    pub item_id: i32,
    pub option_value: String,
    pub option_display: String,
    pub option_order: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateItemOptionReply {
    pub data: ItemOption,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct CreateItemUnitRequest {
    pub item_id: i32,
    pub name: String,
    pub unit_order: i32,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemUnit {
    pub id: i32,
    pub name: String,
    pub unit_order: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateItemUnitReply {
    pub data: ItemUnit,
}
