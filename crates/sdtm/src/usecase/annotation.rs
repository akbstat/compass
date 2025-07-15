use crate::{
    errors::Result,
    repository::{
        AnnotationRepository,
        dao::{NewAnnotation, UpdateAnnotation},
    },
};
use apis::sdtm::annotation::{
    Annotation, AnnotationInForm, AnnotationKind, AnnotationVersion, CreateAnnotationRequest,
    CreateAnnotationVersionRequest, CreateFormDomainRequest, FormDomain, FormVariable,
    ListAnnotationByFormRequest, ListFormDomainRequest, ListVariableByFormRequest,
    ModifyAnnotationVersionRequest, UpdateAnnootationRequest, VariableBinding,
};
use futures::future::join_all;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AnnotationUsecase {
    repo: AnnotationRepository,
}

impl AnnotationUsecase {
    pub fn new(pool: Arc<PgPool>) -> AnnotationUsecase {
        let repo = AnnotationRepository::new(pool);
        AnnotationUsecase { repo }
    }

    pub async fn create_annotation_version(
        &self,
        request: &CreateAnnotationVersionRequest,
    ) -> Result<AnnotationVersion> {
        let data = self.repo.create_annotation_version(request).await?.into();
        Ok(data)
    }

    pub async fn list_annotation_versions(
        &self,
        project_version_id: i32,
    ) -> Result<Vec<AnnotationVersion>> {
        let versions = self
            .repo
            .list_annotation_versions(project_version_id)
            .await?;
        Ok(versions.into_iter().map(Into::into).collect())
    }

    pub async fn modify_annotation_version(
        &self,
        id: i32,
        request: &ModifyAnnotationVersionRequest,
    ) -> Result<AnnotationVersion> {
        let data = self.repo.modify_annotation_version(id, request).await?;
        Ok(data.into())
    }

    pub async fn remove_annotation_version(&self, id: i32) -> Result<()> {
        self.repo.remove_annotation_version(id).await?;
        Ok(())
    }

    pub async fn create_form_domain(
        &self,
        request: &CreateFormDomainRequest,
    ) -> Result<FormDomain> {
        let data = self.repo.create_form_domain(request).await?.into();
        Ok(data)
    }

    pub async fn list_form_domains(
        &self,
        request: &ListFormDomainRequest,
    ) -> Result<Vec<FormDomain>> {
        let domains = self.repo.list_form_domains(request).await?;
        Ok(domains.into_iter().map(Into::into).collect())
    }

    pub async fn list_variable_by_form(
        &self,
        request: &ListVariableByFormRequest,
    ) -> Result<Vec<FormVariable>> {
        let results = self
            .list_form_domains(&ListFormDomainRequest {
                annotation_version_id: request.annotation_version_id,
                form_id: request.form_id,
            })
            .await?
            .into_iter()
            .map(|domain| self.list_variables_by_domain(domain.id))
            .collect::<Vec<_>>();
        let mut variables = vec![];
        join_all(results).await.into_iter().for_each(|reply| {
            if let Ok(vars) = reply {
                vars.into_iter().for_each(|variable| {
                    variables.push(variable);
                });
            }
        });
        Ok(variables)
    }

    pub async fn remove_form_domain(&self, id: i32) -> Result<()> {
        self.repo.remove_form_domain(id).await?;
        let variable_ids = self
            .list_variables_by_domain(id)
            .await?
            .into_iter()
            .map(|v| v.id)
            .collect::<Vec<i32>>();
        self.remove_annotations_by_variable_ids(&variable_ids)
            .await?;
        Ok(())
    }

    pub async fn list_annotation_by_form(
        &self,
        request: &ListAnnotationByFormRequest,
    ) -> Result<AnnotationInForm> {
        let rows = self.repo.list_annotation_by_form(request).await?;
        let mut annotations = AnnotationInForm {
            form: vec![],
            item: vec![],
            value: vec![],
            unit: vec![],
            option: vec![],
        };
        for row in rows.into_iter() {
            let row: Annotation = row.into();
            match row.kind {
                AnnotationKind::Form => annotations.form.push(row),
                AnnotationKind::Item => annotations.item.push(row),
                AnnotationKind::Value => annotations.value.push(row),
                AnnotationKind::Unit => annotations.unit.push(row),
                AnnotationKind::Option => annotations.option.push(row),
                AnnotationKind::Unknown => {}
            }
        }
        Ok(annotations)
    }

    pub async fn create_annotation(&self, request: &CreateAnnotationRequest) -> Result<Annotation> {
        let variable_id = match &request.variable {
            Some(binding) => self.create_or_get_form_variable(binding).await?.id,
            None => -1,
        };
        let data = self
            .repo
            .create_annotation(&NewAnnotation {
                annotation_version_id: request.annotation_version_id,
                form_id: request.form_id,
                variable_id,
                source_id: request.location.source_id,
                kind: match request.location.kind {
                    apis::sdtm::annotation::AnnotationKind::Form => 0,
                    apis::sdtm::annotation::AnnotationKind::Item => 1,
                    apis::sdtm::annotation::AnnotationKind::Value => 2,
                    apis::sdtm::annotation::AnnotationKind::Unit => 3,
                    apis::sdtm::annotation::AnnotationKind::Option => 4,
                    _ => -1, // Unknown kind
                },
                annotation_display: request.annotation_display.to_owned(),
                assign: request.assign,
            })
            .await?
            .into();
        Ok(data)
    }

    pub async fn update_annotation(
        &self,
        id: i32,
        request: &UpdateAnnootationRequest,
    ) -> Result<Annotation> {
        let variable = match &request.not_submit {
            false => match &request.variable {
                Some(variable_binding) => {
                    let variable = self.create_or_get_form_variable(&variable_binding).await?;
                    if variable_binding.supp.ne(&variable.supp) {
                        self.repo
                            .update_form_variable(variable.id, variable_binding.supp)
                            .await?;
                    }
                    Some(variable)
                }
                None => None,
            },
            true => None,
        };
        let data = self
            .repo
            .update_annotation(
                id,
                &UpdateAnnotation {
                    variable_id: match &variable {
                        Some(variable) => Some(variable.id),
                        None => None,
                    },
                    annotation_display: request.annotation_display.to_owned(),
                    assign: request.assign,
                },
            )
            .await?;
        Ok(data.into())
    }

    pub async fn remove_annotation(&self, id: i32) -> Result<()> {
        self.repo.remove_annoation(id).await
    }

    async fn list_variables_by_domain(&self, domain_id: i32) -> Result<Vec<FormVariable>> {
        let variables = self.repo.list_variables_by_domain(domain_id).await?;
        Ok(variables.into_iter().map(Into::into).collect())
    }

    async fn remove_annotations_by_variable_ids(&self, variable_ids: &[i32]) -> Result<()> {
        self.repo
            .remove_annotations_by_variable_ids(&variable_ids)
            .await?;
        Ok(())
    }

    pub async fn create_or_get_form_variable(
        &self,
        variable: &VariableBinding,
    ) -> Result<FormVariable> {
        let variable = match self
            .repo
            .get_form_variable_by_name(variable.domain_id, &variable.variable_name)
            .await?
        {
            Some(variable) => variable,
            None => self.repo.create_form_variable(&variable).await?,
        };
        Ok(variable.into())
    }
}
