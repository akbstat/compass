use crate::{
    errors::Result,
    repository::dao::{AnnotationRow, AnnotationVersionRow, FormDomainRow, FormVariableRow, NewAnnotation, UpdateAnnotation},
};
use apis::sdtm::annotation::{ CreateAnnotationVersionRequest, CreateFormDomainRequest, ListAnnotationByFormRequest, ListFormDomainRequest, ModifyAnnotationVersionRequest, VariableBinding
};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AnnotationRepository {
    pool: Arc<PgPool>,
}

impl AnnotationRepository {
    pub fn new(pool: Arc<PgPool>) -> AnnotationRepository {
        AnnotationRepository { pool }
    }

    pub async fn create_annotation_version(
        &self,
        request: &CreateAnnotationVersionRequest,
    ) -> Result<AnnotationVersionRow> {
        let row: AnnotationVersionRow = sqlx::query_as(
            r#"
INSERT INTO annotation_version (project_version_id, name, description)
VALUES ($1, $2, $3)
RETURNING id, project_version_id, name, description
        "#,
        )
        .bind(request.project_version_id)
        .bind(&request.name)
        .bind(&request.description)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn list_annotation_versions(
        &self,
        project_version_id: i32,
    ) -> Result<Vec<AnnotationVersionRow>> {
        let rows: Vec<AnnotationVersionRow> = sqlx::query_as(
            r#"SELECT id, project_version_id, name, description
FROM annotation_version
WHERE project_version_id = $1
        "#,
        )
        .bind(project_version_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn modify_annotation_version(
        &self,
        id: i32,
        request: &ModifyAnnotationVersionRequest,
    ) -> Result<AnnotationVersionRow> {
        let row: AnnotationVersionRow = sqlx::query_as(r#"
        UPDATE annotation_version SET name = $1, description = $2 WHERE id = $3
        RETURNING id, project_version_id, name, description
        "#).bind(&request.name).bind(&request.description).bind(id).fetch_one(self.pool.as_ref()).await?;
        Ok(row)
    }

    pub async fn remove_annotation_version(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM annotation_version WHERE id = $1")
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn create_form_domain(
        &self,
        request: &CreateFormDomainRequest,
    ) -> Result<FormDomainRow> {
        let row: FormDomainRow = sqlx::query_as(
            r#"INSERT INTO form_domain (annotation_version_id, form_id, name, description)
VALUES ($1, $2, $3, $4)
RETURNING id, annotation_version_id, form_id, name, description
        "#,
        )
        .bind(request.annotation_version_id)
        .bind(request.form_id)
        .bind(&request.name)
        .bind(&request.description)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn list_form_domains(
        &self,
        request: &ListFormDomainRequest,
    ) -> Result<Vec<FormDomainRow>> {
        let rows: Vec<FormDomainRow> = sqlx::query_as(
            r#"SELECT id, annotation_version_id, form_id, name, description
FROM form_domain
WHERE form_id = $1 AND annotation_version_id = $2
        "#,
        )
        .bind(request.form_id)
        .bind(request.annotation_version_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn remove_form_domain(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM form_domain WHERE id = $1")
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn list_variables_by_domain(&self, domain_id: i32) -> Result<Vec<FormVariableRow>> {
        let rows: Vec<FormVariableRow> = sqlx::query_as(
            r#"SELECT id, domain_id, name, supp
FROM form_variable
WHERE domain_id = $1
        "#,
        )
        .bind(domain_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn remove_annotations_by_variable_ids(&self, variable_ids: &[i32]) -> Result<()> {
        if variable_ids.is_empty() {
            return Ok(());
        }
        // Build a query with the correct number of placeholders
        let placeholders = (1..=variable_ids.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(", ");
        let query = format!(
            "DELETE FROM annotation WHERE variable_id IN ({})",
            placeholders
        );
        let mut q = sqlx::query(&query);
        for id in variable_ids {
            q = q.bind(id);
        }
        q.execute(self.pool.as_ref()).await?;
        Ok(())
    }

      pub async fn list_annotation_by_form(&self, request: &ListAnnotationByFormRequest) -> Result<Vec<AnnotationRow>> {
        let rows: Vec<AnnotationRow> = sqlx::query_as(
            r#"
SELECT id, annotation_version_id, form_id, variable_id, source_id, kind, annotation_display, assign
FROM annotation
WHERE form_id = $1 AND annotation_version_id = $2
            "#
        )
        .bind(request.form_id)
        .bind(request.annotation_version_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn create_annotation(
        &self,
        request: &NewAnnotation,
    ) -> Result<AnnotationRow> {
        let row: AnnotationRow = sqlx::query_as(
            r#"INSERT INTO annotation (annotation_version_id, form_id, variable_id, source_id, kind, annotation_display, assign)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING id, annotation_version_id, form_id, variable_id, source_id,
kind, annotation_display, assign
        "#,
        )
        .bind(request.annotation_version_id)
        .bind(request.form_id)
        .bind(request.variable_id)
        .bind(request.source_id)
        .bind(request.kind) 
        .bind(&request.annotation_display)
        .bind(request.assign)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn update_annotation(
        &self,
        id: i32,
        imcoming: &UpdateAnnotation,
    ) -> Result<AnnotationRow> {
        if let Some(variable_id) = imcoming.variable_id {
            sqlx::query(
                r#"
UPDATE annotation
SET variable_id = $1
WHERE id = $2
                "#
            ).bind(&variable_id).bind(id).execute(self.pool.as_ref()).await?;
        }

        let row: AnnotationRow = sqlx::query_as(
            r#"
UPDATE annotation
SET annotation_display = $1,
    assign = $2
WHERE id = $3
RETURNING id, annotation_version_id, form_id, variable_id, source_id, kind, annotation_display, assign
            "#
        )
        .bind(&imcoming.annotation_display)
        .bind(imcoming.assign)
        .bind(id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn remove_annoation(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM annotation WHERE id = $1")
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

     pub async fn get_form_variable_by_name(&self, domain_id: i32, name: &str) -> Result<Option<FormVariableRow>> {
        let row = sqlx::query_as::<_, FormVariableRow>(
            r#"
SELECT id, domain_id, name, supp
FROM form_variable
WHERE domain_id = $1 AND name = $2
            "#
        )
        .bind(domain_id)
        .bind(name)
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_form_variable(&self, variable: &VariableBinding) -> Result<FormVariableRow> {
        let row: FormVariableRow = sqlx::query_as(
            r#"
INSERT INTO form_variable (domain_id, name, supp)
VALUES ($1, $2, $3)
RETURNING id, domain_id, name, supp
            "#
        )
        .bind(variable.domain_id)
        .bind(&variable.variable_name)
        .bind(variable.supp)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn update_form_variable(&self, id: i32, supp: bool) -> Result<FormVariableRow> {
        let row: FormVariableRow = sqlx::query_as(
            r#"
UPDATE form_variable
SET supp = $1
WHERE id = $2
RETURNING id, domain_id, name, supp
            "#
        )
        .bind(supp)
        .bind(id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

}
