use super::dao::{
    FormRow, ItemOptionRow, ItemRow, ItemTypeRow, ItemUnitRow, ProjectRow, ProjectVersionRow,
};
use crate::errors::Result;
use apis::sdtm::rawdata::{
    CreateFormRequest, CreateItemOptionRequest, CreateItemRequest, CreateItemUnitRequest,
};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct RawdataRepository {
    pool: Arc<PgPool>,
}

impl RawdataRepository {
    pub fn new(pool: Arc<PgPool>) -> RawdataRepository {
        RawdataRepository { pool }
    }

    pub async fn find_one_project(&self, project: &str) -> Result<Option<ProjectRow>> {
        let row: Option<ProjectRow> = sqlx::query_as(
            r#"
SELECT id, name
FROM project
WHERE name = $1        
        "#,
        )
        .bind(project)
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn list_project_versions(&self, project_id: i32) -> Result<Vec<ProjectVersionRow>> {
        let rows: Vec<ProjectVersionRow> = sqlx::query_as(
            r#"
SELECT id, name
FROM project_version
WHERE project_id = $1
        "#,
        )
        .bind(project_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn list_forms(&self, version_id: i32) -> Result<Vec<FormRow>> {
        let rows: Vec<FormRow> = sqlx::query_as(
            r#"
SELECT id, version_id, name, description, form_order
FROM form
WHERE version_id = $1
ORDER BY form_order
        "#,
        )
        .bind(version_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn get_form_by_id(&self, id: i32) -> Result<Option<FormRow>> {
        let row = sqlx::query_as(
            r#"
SELECT id, version_id, name, description, form_order
FROM form
WHERE id = $1
LIMIT 1
        "#,
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn list_items(&self, form_id: i32) -> Result<Vec<ItemRow>> {
        let rows = sqlx::query_as(
            r#"
SELECT id, form_id, name, label, item_type_id, item_order, item_repeat_index, item_default_value
FROM item
WHERE form_id = $1
ORDER BY item_order, item_repeat_index
        "#,
        )
        .bind(form_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn list_item_options(&self, item_ids: &[i32]) -> Result<Vec<ItemOptionRow>> {
        let item_ids = item_ids
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let sql_syntax = format!(
            "SELECT id, item_id, option_value, option_display, option_order FROM item_option WHERE item_id IN ({})",
            item_ids,
        );
        let rows = sqlx::query_as(&sql_syntax)
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(rows)
    }

    pub async fn list_item_units(&self, item_ids: &[i32]) -> Result<Vec<ItemUnitRow>> {
        let item_ids = item_ids
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let sql_syntax = format!(
            "SELECT id, item_id, name, unit_order FROM item_unit WHERE item_id IN ({})",
            item_ids,
        );
        let rows = sqlx::query_as(&sql_syntax)
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(rows)
    }

    pub async fn list_item_types(&self) -> Result<Vec<ItemTypeRow>> {
        let rows = sqlx::query_as(
            r#"
SELECT id, name
FROM item_type
        "#,
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn create_project(&self, name: &str) -> Result<ProjectRow> {
        let row: ProjectRow = sqlx::query_as(
            r#"
INSERT INTO project (name) VALUES ($1)
RETURNING id, name
        "#,
        )
        .bind(name)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_project_version(
        &self,
        name: &str,
        project_id: i32,
    ) -> Result<ProjectVersionRow> {
        let row: ProjectVersionRow = sqlx::query_as(
            r#"
INSERT INTO project_version (project_id, name) VALUES ($1, $2)
RETURNING id, project_id, name
        "#,
        )
        .bind(project_id)
        .bind(name)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn modify_project_version(&self, id: i32, name: &str) -> Result<ProjectVersionRow> {
        let row: ProjectVersionRow = sqlx::query_as(
            r#"
UPDATE project_version SET name = $1 WHERE id = $2
RETURNING id, project_id, name
        "#,
        )
        .bind(name)
        .bind(id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_form(&self, request: &CreateFormRequest) -> Result<FormRow> {
        let row: FormRow = sqlx::query_as(
            r#"
INSERT INTO form (version_id, name, description, form_order) VALUES ($1, $2, $3, $4)
RETURNING id, name, description, form_order        
        "#,
        )
        .bind(request.version_id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(request.form_order)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_item_type(&self, name: &str) -> Result<ItemTypeRow> {
        let row: ItemTypeRow = sqlx::query_as(
            r#"
INSERT INTO item_type (name) VALUES ($1)
RETURNING id, name
        "#,
        )
        .bind(name)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_item(&self, request: &CreateItemRequest) -> Result<ItemRow> {
        let row: ItemRow = sqlx::query_as(
            r#"
INSERT INTO item (form_id, name, label, item_type_id, item_order, item_repeat_index, item_default_value) VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING id, form_id, name, label, item_type_id, item_order, item_repeat_index, item_default_value
        "#,
        )
        .bind(request.form_id)
        .bind(&request.name)
        .bind(&request.label)
        .bind(request.item_type_id)
        .bind(request.item_order)
        .bind(request.item_repeat_index)
        .bind(&request.item_defualt_value)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_item_option(
        &self,
        request: &CreateItemOptionRequest,
    ) -> Result<ItemOptionRow> {
        let row: ItemOptionRow = sqlx::query_as(
            r#"
INSERT INTO item_option (item_id, option_value, option_display, option_order) VALUES ($1, $2, $3, $4)
RETURNING id, item_id, option_value, option_display, option_order
        "#,
        )
        .bind(request.item_id)
        .bind(&request.option_value)
        .bind(&request.option_display)
        .bind(request.option_order)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_item_unit(&self, request: &CreateItemUnitRequest) -> Result<ItemUnitRow> {
        let row: ItemUnitRow = sqlx::query_as(
            r#"
INSERT INTO item_unit (item_id, name, unit_order) VALUES ($1, $2, $3)
RETURNING id, name, item_id, unit_order
        "#,
        )
        .bind(request.item_id)
        .bind(&request.name)
        .bind(request.unit_order)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }
}
