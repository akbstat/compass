use crate::{errors::Result, repository::RawdataRepository};
use apis::sdtm::rawdata::{
    CreateFormRequest, CreateItemOptionRequest, CreateItemRequest, CreateItemTypeRequest,
    CreateItemUnitRequest, CreateProjectRequest, CreateProjectVersionRequest, FindProjectRequest,
    Form, Item, ItemDetail, ItemOption, ItemType, ItemUnit, ListFormsRequest, ListItemsRequest,
    ListProjectVersionRequest, ModifyProjectVersionRequest, Project, ProjectVersion,
};
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct RawdataUsecase {
    repo: RawdataRepository,
}

impl RawdataUsecase {
    pub fn new(pool: Arc<PgPool>) -> RawdataUsecase {
        let repo = RawdataRepository::new(pool);
        RawdataUsecase { repo }
    }

    pub async fn list_project_versions(
        &self,
        request: &ListProjectVersionRequest,
    ) -> Result<(i32, Vec<ProjectVersion>)> {
        let project_name = format!("{}-{}", request.product, request.trial);
        let project = self.repo.find_one_project(&project_name).await?;
        match project {
            Some(project) => Ok((
                project.id,
                self.repo
                    .list_project_versions(project.id)
                    .await?
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            )),
            None => {
                let project = self
                    .create_project(&CreateProjectRequest { name: project_name })
                    .await?;
                Ok((project.id, vec![]))
            }
        }
    }

    pub async fn find_project(&self, request: &FindProjectRequest) -> Result<Option<Project>> {
        let project_name = format!("{}-{}", request.product, request.trial);
        let project = self.repo.find_one_project(&project_name).await?;
        Ok(project.map(|p| p.into()))
    }

    pub async fn list_forms(&self, request: &ListFormsRequest) -> Result<Vec<Form>> {
        let forms = self.repo.list_forms(request.version_id).await?;
        Ok(forms.into_iter().map(Into::into).collect())
    }

    pub async fn get_form_by_id(&self, id: i32) -> Result<Option<Form>> {
        let form = self.repo.get_form_by_id(id).await?;
        Ok(form.map(|f| f.into()))
    }

    pub async fn get_item_by_id(&self, id: i32) -> Result<Option<Item>> {
        let item = self.repo.get_item_by_id(id).await?;
        Ok(item.map(|i| i.into()))
    }

    pub async fn get_option_by_id(&self, id: i32) -> Result<Option<ItemOption>> {
        let item = self.repo.get_option_by_id(id).await?;
        Ok(item.map(|i| i.into()))
    }

    pub async fn get_unit_by_id(&self, id: i32) -> Result<Option<ItemUnit>> {
        let item = self.repo.get_unit_by_id(id).await?;
        Ok(item.map(|i| i.into()))
    }

    pub async fn list_items(&self, request: &ListItemsRequest) -> Result<Vec<ItemDetail>> {
        let items = self.repo.list_items(request.form_id).await?;
        let item_ids = items.iter().map(|item| item.id).collect::<Vec<i32>>();
        let option_map = self.list_item_options(&item_ids).await?;
        let unit_map = self.list_item_units(&item_ids).await?;
        let item_types = self.list_item_types().await?;
        let type_map = item_types
            .into_iter()
            .map(|item_type| (item_type.id, item_type))
            .collect::<HashMap<i32, ItemType>>();
        Ok(items
            .into_iter()
            .map(|item| ItemDetail {
                id: item.id,
                name: item.name,
                label: item.label,
                item_type: type_map.get(&item.item_type_id).cloned(),
                item_option: option_map.get(&item.id).cloned(),
                item_unit: unit_map.get(&item.id).cloned(),
                item_order: item.item_order,
                item_defualt_value: item.item_default_value,
                item_repeat_index: item.item_repeat_index,
            })
            .collect())
    }

    async fn list_item_options(&self, item_ids: &[i32]) -> Result<HashMap<i32, Vec<ItemOption>>> {
        let options = self.repo.list_item_options(item_ids).await?;
        let mut item_map: HashMap<i32, Vec<ItemOption>> = HashMap::with_capacity(options.len());
        for option in options {
            let item_id = option.item_id;
            match item_map.get_mut(&item_id) {
                Some(options) => {
                    options.push(option.into());
                }
                None => {
                    item_map.insert(item_id, vec![option.into()]);
                }
            }
        }
        Ok(item_map)
    }

    async fn list_item_units(&self, item_ids: &[i32]) -> Result<HashMap<i32, Vec<ItemUnit>>> {
        let units = self.repo.list_item_units(item_ids).await?;
        let mut unit_map: HashMap<i32, Vec<ItemUnit>> = HashMap::with_capacity(units.len());
        for unit in units {
            let item_id = unit.item_id;
            match unit_map.get_mut(&item_id) {
                Some(units) => {
                    units.push(unit.into());
                }
                None => {
                    unit_map.insert(item_id, vec![unit.into()]);
                }
            }
        }
        Ok(unit_map)
    }

    pub async fn list_item_types(&self) -> Result<Vec<ItemType>> {
        let unit = self.repo.list_item_types().await?;
        Ok(unit.into_iter().map(|item_type| item_type.into()).collect())
    }

    async fn create_project(&self, request: &CreateProjectRequest) -> Result<Project> {
        let project = self.repo.create_project(&request.name).await?;
        Ok(project.into())
    }

    pub async fn create_project_version(
        &self,
        request: &CreateProjectVersionRequest,
    ) -> Result<ProjectVersion> {
        let version = self
            .repo
            .create_project_version(&request.name, request.project_id)
            .await?;
        Ok(version.into())
    }

    pub async fn modify_project_version(
        &self,
        id: i32,
        request: &ModifyProjectVersionRequest,
    ) -> Result<ProjectVersion> {
        let version = self.repo.modify_project_version(id, &request.name).await?;
        Ok(version.into())
    }

    pub async fn create_form(&self, request: &CreateFormRequest) -> Result<Form> {
        let form = self.repo.create_form(request).await?;
        Ok(form.into())
    }

    pub async fn create_item_type(&self, request: &CreateItemTypeRequest) -> Result<ItemType> {
        let item_type = self.repo.create_item_type(&request.name).await?;
        Ok(item_type.into())
    }

    pub async fn create_item(&self, request: &CreateItemRequest) -> Result<Item> {
        let item = self.repo.create_item(request).await?;
        Ok(item.into())
    }

    pub async fn create_item_option(
        &self,
        request: &CreateItemOptionRequest,
    ) -> Result<ItemOption> {
        let option = self.repo.create_item_option(request).await?;
        Ok(option.into())
    }

    pub async fn create_item_unit(&self, request: &CreateItemUnitRequest) -> Result<ItemUnit> {
        let unit = self.repo.create_item_unit(request).await?;
        Ok(unit.into())
    }
}
