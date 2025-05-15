pub struct CreateProjectRequest {
    pub name: String,
}

pub struct Project {
    pub id: i32,
    pub name: String,
}

pub struct CreateProjectVersionRequest {
    pub project_id: i32,
    pub name: String,
}

pub struct ProjectVersion {
    pub id: i32,
    pub name: String,
}

pub struct CreateFormRequest {
    pub version_id: i32,
    pub name: String,
    pub description: String,
    pub form_order: i32,
}

#[derive(Debug)]
pub struct Form {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub form_order: i32,
}

pub struct CreateItemTypeRequest {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ItemType {
    pub id: i32,
    pub name: String,
}

pub struct CreateItemRequest {
    pub form_id: i32,
    pub name: String,
    pub label: String,
    pub item_type_id: i32,
    pub item_order: i32,
}

#[derive(Debug)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub label: String,
    pub item_type_id: i32,
    pub item_order: i32,
}

#[derive(Debug)]
pub struct ItemDetail {
    pub id: i32,
    pub name: String,
    pub label: String,
    pub item_type: Option<ItemType>,
    pub item_option: Option<Vec<ItemOption>>,
    pub item_unit: Option<Vec<ItemUnit>>,
    pub item_order: i32,
}

pub struct CreateItemOptionRequest {
    pub item_id: i32,
    pub option_value: String,
    pub option_display: String,
    pub option_order: i32,
}

#[derive(Debug, Clone)]
pub struct ItemOption {
    pub id: i32,
    pub option_value: String,
    pub option_display: String,
    pub option_order: i32,
}

pub struct CreateItemUnitRequest {
    pub item_id: i32,
    pub name: String,
    pub unit_order: i32,
}

#[derive(Debug, Clone)]
pub struct ItemUnit {
    pub id: i32,
    pub name: String,
    pub unit_order: i32,
}

pub struct ListFormsRequest {
    pub version_id: i32,
}

pub struct ListItemsRequest {
    pub form_id: i32,
}
