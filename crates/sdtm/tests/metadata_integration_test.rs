use apis::sdtm::metadata::{
    CreateCoreRequest, CreateLanguageRequest, CreateRoleRequest, CreateSdtmDomainRequest,
    CreateSdtmVariableRequest, CreateSdtmVersionRequest, CreateTypeRequest,
};
use dotenv::dotenv;
use sdtm::MetadataUsecase;
use sqlx::PgPool;
use std::{env, error::Error, sync::Arc};

#[tokio::test]
async fn metadata_integration_test() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = Arc::new(PgPool::connect(&database_url).await?);
    let uc = MetadataUsecase::new(pool);

    // create data
    let create_language_request = CreateLanguageRequest {
        name: "en-US".into(),
    };
    let lang = uc.create_language(&create_language_request).await?;
    assert_eq!(create_language_request.name, lang.name);

    let create_type_request = CreateTypeRequest {
        lang_id: lang.id,
        name: "Char".into(),
    };
    let variable_type = uc.create_type(&create_type_request).await?;
    assert_eq!(create_type_request.name, variable_type.name);

    let create_core_request = CreateCoreRequest {
        name: "Req".into(),
        lang_id: lang.id,
    };
    let variable_core = uc.create_core(&create_core_request).await?;
    assert_eq!(create_core_request.name, variable_core.name);

    let create_role_request = CreateRoleRequest {
        name: "Topic".into(),
        lang_id: lang.id,
    };
    let variable_role = uc.create_role(&create_role_request).await?;
    assert_eq!(create_role_request.name, variable_role.name);

    let create_sdtm_version_request = CreateSdtmVersionRequest {
        name: "3.4".into(),
        lang_id: lang.id,
    };
    let sdtm_version = uc.create_sdtm_version(&create_sdtm_version_request).await?;
    assert_eq!(create_sdtm_version_request.name, sdtm_version.name);

    let create_sdtm_domain_request = CreateSdtmDomainRequest {
        name: "DM".into(),
        description: "Demographics".into(),
        version_id: sdtm_version.id,
    };
    let sdtm_domain = uc.create_sdtm_domain(&create_sdtm_domain_request).await?;
    assert_eq!(create_sdtm_domain_request.name, sdtm_domain.name);

    let create_sdtm_variable_request = CreateSdtmVariableRequest {
        name: "SUBJID".to_string(),
        label: "Subject Identifier for the Study".to_string(),
        domain_id: sdtm_domain.id,
        variable_type_id: variable_type.id,
        codelist: "".to_string(),
        variable_core_id: variable_core.id,
        variable_role_id: variable_role.id,
        variable_order: 2,
    };
    let sdtm_variable = uc
        .create_sdtm_variable(&create_sdtm_variable_request)
        .await?;
    assert_eq!(sdtm_variable.name, create_sdtm_variable_request.name);
    assert_eq!(sdtm_variable.variable_core, variable_core.name);
    assert_eq!(sdtm_variable.variable_role, variable_role.name);
    assert_eq!(sdtm_variable.variable_type, variable_type.name);

    // query data
    let lang_list = uc.list_languages().await?;
    assert_eq!(lang_list.len(), 1);
    let query_lang = lang_list.first().unwrap();
    assert_eq!(query_lang.name, lang.name);

    let version_list = uc.list_sdtm_versions(query_lang.id).await?;
    assert_eq!(version_list.len(), 1);
    let query_version = version_list.first().unwrap();
    assert_eq!(query_version.name, sdtm_version.name);

    let domain_list = uc.list_sdtm_domains(query_version.id).await?;
    assert_eq!(domain_list.len(), 1);
    let query_domain = domain_list.first().unwrap();
    assert_eq!(query_domain.name, sdtm_domain.name);

    let variable_list = uc.list_sdtm_variables(query_domain.id).await?;
    assert_eq!(variable_list.len(), 1);
    let query_variable = variable_list.first().unwrap();
    assert_eq!(query_variable.name, sdtm_variable.name);

    Ok(())
}
