use apis::sdtm::rawdata::{Form, Item, ItemOption, ItemType, ItemUnit, Project, ProjectVersion};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct ProjectRow {
    pub id: i32,
    pub name: String,
}

impl Into<Project> for ProjectRow {
    fn into(self) -> Project {
        Project {
            id: self.id,
            name: self.name,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct ProjectVersionRow {
    pub id: i32,
    pub name: String,
}

impl Into<ProjectVersion> for ProjectVersionRow {
    fn into(self) -> ProjectVersion {
        ProjectVersion {
            id: self.id,
            name: self.name,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct FormRow {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub form_order: i32,
}

impl Into<Form> for FormRow {
    fn into(self) -> Form {
        Form {
            id: self.id,
            name: self.name,
            description: self.description,
            form_order: self.form_order,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct ItemTypeRow {
    pub id: i32,
    pub name: String,
}

impl Into<ItemType> for ItemTypeRow {
    fn into(self) -> ItemType {
        ItemType {
            id: self.id,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct ItemRow {
    pub id: i32,
    pub form_id: i32,
    pub name: String,
    pub label: String,
    pub item_type_id: i32,
    pub item_order: i32,
}

impl Into<Item> for ItemRow {
    fn into(self) -> Item {
        Item {
            id: self.id,
            name: self.name,
            label: self.label,
            item_type_id: self.item_type_id,
            item_order: self.item_order,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct ItemOptionRow {
    pub id: i32,
    pub item_id: i32,
    pub option_value: String,
    pub option_display: String,
    pub option_order: i32,
}

impl Into<ItemOption> for ItemOptionRow {
    fn into(self) -> ItemOption {
        ItemOption {
            id: self.id,
            option_value: self.option_value,
            option_display: self.option_display,
            option_order: self.option_order,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct ItemUnitRow {
    pub id: i32,
    pub item_id: i32,
    pub name: String,
    pub unit_order: i32,
}

impl Into<ItemUnit> for ItemUnitRow {
    fn into(self) -> ItemUnit {
        ItemUnit {
            id: self.id,
            name: self.name,
            unit_order: self.unit_order,
        }
    }
}
