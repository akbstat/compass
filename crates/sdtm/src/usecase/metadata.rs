use crate::{errors::Result, repository::MetadataRepository};
use apis::sdtm::metadata::{
    CreateCoreRequest, CreateLanguageRequest, CreateRoleRequest, CreateSdtmDomainRequest,
    CreateSdtmVariableRequest, CreateSdtmVersionRequest, CreateTypeRequest, Language, SdtmDomain,
    SdtmVariable, SdtmVersion, VariableCore, VariableRole, VariableType,
};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct MetadataUsecase {
    repo: MetadataRepository,
}

impl MetadataUsecase {
    pub fn new(pool: Arc<PgPool>) -> MetadataUsecase {
        let repo = MetadataRepository::new(pool);
        MetadataUsecase { repo }
    }

    pub async fn list_languages(&self) -> Result<Vec<Language>> {
        let rows = self.repo.list_lanaguages().await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn list_sdtm_versions(&self, lang_id: i32) -> Result<Vec<SdtmVersion>> {
        let rows = self.repo.list_sdtm_versions(lang_id).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn list_sdtm_domains(&self, version_id: i32) -> Result<Vec<SdtmDomain>> {
        let rows = self.repo.list_sdtm_domains(version_id).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn list_sdtm_variables(&self, domain_id: i32) -> Result<Vec<SdtmVariable>> {
        let rows = self.repo.list_sdtm_variables(domain_id).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
    pub async fn create_language(&self, request: &CreateLanguageRequest) -> Result<Language> {
        let language = self.repo.create_language(&request.name).await?;
        Ok(language.into())
    }

    pub async fn create_type(&self, request: &CreateTypeRequest) -> Result<VariableType> {
        let variable_type = self
            .repo
            .create_type(&request.name, request.lang_id)
            .await?;
        Ok(variable_type.into())
    }

    pub async fn create_core(&self, request: &CreateCoreRequest) -> Result<VariableCore> {
        let core = self
            .repo
            .create_core(&request.name, request.lang_id)
            .await?;
        Ok(core.into())
    }

    pub async fn create_role(&self, request: &CreateRoleRequest) -> Result<VariableRole> {
        let role = self
            .repo
            .create_role(&request.name, request.lang_id)
            .await?;
        Ok(role.into())
    }

    pub async fn create_sdtm_version(
        &self,
        request: &CreateSdtmVersionRequest,
    ) -> Result<SdtmVersion> {
        let sdtm_version = self
            .repo
            .create_sdtm_version(&request.name, request.lang_id)
            .await?;
        Ok(sdtm_version.into())
    }

    pub async fn create_sdtm_domain(
        &self,
        request: &CreateSdtmDomainRequest,
    ) -> Result<SdtmDomain> {
        let sdtm_domain = self.repo.create_sdtm_domain(request).await?;
        Ok(sdtm_domain.into())
    }

    pub async fn create_sdtm_variable(
        &self,
        request: &CreateSdtmVariableRequest,
    ) -> Result<SdtmVariable> {
        let variable_id = self.repo.create_sdtm_variable(request).await?;
        let sdtm_variable = self.repo.get_sdtm_variable(variable_id).await?;
        Ok(sdtm_variable.into())
    }
}
