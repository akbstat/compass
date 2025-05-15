use apis::sdtm::rawdata::{
    CreateFormRequest, CreateItemOptionRequest, CreateItemRequest, CreateItemTypeRequest,
    CreateItemUnitRequest, CreateProjectRequest, CreateProjectVersionRequest, ListFormsRequest,
    ListItemsRequest,
};
use dotenv::dotenv;
use sdtm::RawdataUsecase;
use sqlx::PgPool;
use std::error::Error;
use std::{env, sync::Arc};

#[tokio::test]
async fn rawdata_integration_test() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = Arc::new(PgPool::connect(&database_url).await?);
    let uc = RawdataUsecase::new(Arc::clone(&pool));

    let create_project_request = CreateProjectRequest {
        name: "akxxx-yyy".into(),
    };
    let project = uc.create_project(&create_project_request).await?;
    assert_eq!(create_project_request.name, project.name);

    let create_project_version_request = CreateProjectVersionRequest {
        project_id: project.id,
        name: "v0.1".into(),
    };
    let project_version = uc
        .create_project_version(&create_project_version_request)
        .await?;
    assert_eq!(project_version.name, create_project_version_request.name);

    let create_form_request = CreateFormRequest {
        version_id: project_version.id,
        name: "AE".into(),
        description: "AE / SAE".into(),
        form_order: 0,
    };
    let form = uc.create_form(&create_form_request).await?;
    assert_eq!(create_form_request.name, form.name);

    let create_item_type_request = CreateItemTypeRequest {
        name: "Text".into(),
    };
    let item_type = uc.create_item_type(&create_item_type_request).await?;
    assert_eq!(item_type.name, create_item_type_request.name);

    let create_item_request = CreateItemRequest {
        form_id: form.id,
        name: "AEYN".into(),
        label: "Adverse event happend?".into(),
        item_type_id: item_type.id,
        item_order: 0,
    };
    let item = uc.create_item(&create_item_request).await?;
    assert_eq!(item.name, create_item_request.name);
    assert_eq!(item.label, create_item_request.label);
    assert_eq!(item.item_type_id, create_item_request.item_type_id);

    let create_item_option_request = CreateItemOptionRequest {
        item_id: item.id,
        option_value: "Y".into(),
        option_display: "Yes".into(),
        option_order: 0,
    };
    let option = uc.create_item_option(&create_item_option_request).await?;
    assert_eq!(
        option.option_display,
        create_item_option_request.option_display
    );
    assert_eq!(option.option_value, create_item_option_request.option_value);
    uc.create_item_option(&CreateItemOptionRequest {
        item_id: item.id,
        option_value: "N".into(),
        option_display: "No".into(),
        option_order: 1,
    })
    .await?;

    let create_item_unit_request = CreateItemUnitRequest {
        item_id: item.id,
        name: "demo".into(),
        unit_order: 0,
    };
    let unit = uc.create_item_unit(&create_item_unit_request).await?;
    assert_eq!(unit.name, create_item_unit_request.name);

    let list_forms_request = ListFormsRequest {
        version_id: project_version.id,
    };
    let forms = uc.list_forms(&list_forms_request).await?;
    assert_eq!(forms.len(), 1);
    assert_eq!(forms.first().unwrap().name, form.name);

    let list_items_request = ListItemsRequest {
        form_id: forms.first().unwrap().id,
    };
    let form_items = uc.list_items(&list_items_request).await?;
    assert_eq!(form_items.len(), 1);

    // verify item detail
    let form_detail = form_items.first().unwrap();
    assert_eq!(form_detail.id, item.id);
    assert_eq!(form_detail.name, item.name);
    assert_eq!(form_detail.label, item.label);
    assert_eq!(
        form_detail
            .item_type
            .clone()
            .expect("Missing item type")
            .name,
        item_type.name
    );
    assert_eq!(
        form_detail.item_unit.clone().expect("Missing Unit").len(),
        1
    );
    assert_eq!(
        form_detail
            .item_option
            .clone()
            .expect("Missing Option")
            .len(),
        2
    );
    Ok(())
}
