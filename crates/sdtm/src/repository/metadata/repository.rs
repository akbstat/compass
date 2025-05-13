use super::dao::{
    LanguageRow, SdtmDomainRow, SdtmVariableViewRow, SdtmVersionRow, VariableCoreRow,
    VariableRoleRow, VariableTypeRow,
};
use crate::errors::Result;
use apis::sdtm::metadata::{CreateSdtmDomainRequest, CreateSdtmVariableRequest};
use sqlx::PgPool;
use std::sync::Arc;

pub struct MetadataRepository {
    pool: Arc<PgPool>,
}

impl MetadataRepository {
    pub fn new(pool: Arc<PgPool>) -> MetadataRepository {
        MetadataRepository { pool }
    }

    pub async fn list_lanaguages(&self) -> Result<Vec<LanguageRow>> {
        let rows: Vec<LanguageRow> = sqlx::query_as(
            r#"
SELECT id, name
FROM lang
        "#,
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn list_sdtm_versions(&self, lang_id: i32) -> Result<Vec<SdtmVersionRow>> {
        let rows: Vec<SdtmVersionRow> = sqlx::query_as(
            r#"
SELECT id, name
FROM sdtm_version
WHERE lang_id = $1
        "#,
        )
        .bind(lang_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn list_sdtm_domains(&self, version_id: i32) -> Result<Vec<SdtmDomainRow>> {
        let rows: Vec<SdtmDomainRow> = sqlx::query_as(
            r#"
SELECT id, name, description
FROM sdtm_domain
WHERE version_id = $1
        "#,
        )
        .bind(version_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn list_sdtm_variables(&self, domain_id: i32) -> Result<Vec<SdtmVariableViewRow>> {
        let rows: Vec<SdtmVariableViewRow> = sqlx::query_as(r#"
SELECT domain_id, variable_id, variable_name, variable_label, variable_type, codelist, variable_core, variable_role, variable_order
FROM sdtm_variable_view
WHERE domain_id = $1  
        "#)
            .bind(domain_id)
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(rows)
    }

    pub async fn create_language(&self, language: &str) -> Result<LanguageRow> {
        let row: LanguageRow = sqlx::query_as(
            r#"
INSERT INTO lang (name) 
VALUES ($1)
RETURNING id, name;
        "#,
        )
        .bind(language)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_type(&self, type_name: &str, lang_id: i32) -> Result<VariableTypeRow> {
        let row: VariableTypeRow = sqlx::query_as(
            r#"
INSERT INTO variable_type (name, lang_id) 
VALUES ($1, $2)
RETURNING id, name;
        "#,
        )
        .bind(type_name)
        .bind(lang_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_core(&self, core_name: &str, lang_id: i32) -> Result<VariableCoreRow> {
        let row: VariableCoreRow = sqlx::query_as(
            r#"
INSERT INTO variable_core (name, lang_id)
VALUES ($1, $2)
RETURNING id, name;
        "#,
        )
        .bind(core_name)
        .bind(lang_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_role(&self, role_name: &str, lang_id: i32) -> Result<VariableRoleRow> {
        let row: VariableRoleRow = sqlx::query_as(
            r#"
INSERT INTO variable_role (name, lang_id)
VALUES ($1, $2)
RETURNING id, name;
        "#,
        )
        .bind(role_name)
        .bind(lang_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_sdtm_version(
        &self,
        version_name: &str,
        lang_id: i32,
    ) -> Result<SdtmVersionRow> {
        let row: SdtmVersionRow = sqlx::query_as(
            r#"
INSERT INTO sdtm_version (name, lang_id)
VALUES ($1, $2)
RETURNING id, name;
        "#,
        )
        .bind(version_name)
        .bind(lang_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_sdtm_domain(
        &self,
        request: &CreateSdtmDomainRequest,
    ) -> Result<SdtmDomainRow> {
        let row: SdtmDomainRow = sqlx::query_as(
            r#"
INSERT INTO sdtm_domain (name, description, version_id)
VALUES ($1, $2, $3)
RETURNING id, name, description;
        "#,
        )
        .bind(&request.name)
        .bind(&request.description)
        .bind(request.version_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    pub async fn create_sdtm_variable(&self, request: &CreateSdtmVariableRequest) -> Result<i32> {
        let CreateSdtmVariableRequest {
            name,
            label,
            domain_id,
            variable_type_id,
            codelist,
            variable_core_id,
            variable_role_id,
            variable_order,
        } = request;
        let row: (i32,) = sqlx::query_as(r#"
INSERT INTO sdtm_variable (name, label, domain_id, variable_type_id, codelist, variable_core_id, variable_role_id, variable_order)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
RETURNING id;
        "#)
        .bind(name)
        .bind(label)
        .bind(domain_id)
        .bind(variable_type_id)
        .bind(codelist)
        .bind(variable_core_id)
        .bind(variable_role_id)
        .bind(variable_order)
        .fetch_one(self.pool.as_ref()).await?;
        Ok(row.0)
    }

    pub async fn get_sdtm_variable(&self, id: i32) -> Result<SdtmVariableViewRow> {
        let row: SdtmVariableViewRow = sqlx::query_as(
            r#"
SELECT domain_id, variable_id, variable_name, variable_label, variable_type, codelist, variable_core, variable_role, variable_order
FROM sdtm_variable_view
WHERE variable_id = $1        
        "#,
        )
        .bind(id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }
}
