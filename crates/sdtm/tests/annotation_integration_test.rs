use apis::sdtm::{
    annotation::{
        AnnotationKind, AnnotationLocation, CreateAnnotationRequest,
        CreateAnnotationVersionRequest, CreateFormDomainRequest, ListAnnotationByFormRequest,
        UpdateAnnootationRequest, VariableBinding,
    },
    rawdata::{ListFormsRequest, ListProjectVersionRequest},
};
use dotenv::dotenv;
use sdtm::{AnnotationUsecase, RawdataUsecase};
use sqlx::PgPool;
use std::{env, error::Error, sync::Arc};

#[tokio::test]
async fn annotation_interation_test() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = Arc::new(PgPool::connect(&database_url).await?);
    let rawdata_usecase = RawdataUsecase::new(Arc::clone(&pool));
    let annotation_usecase = AnnotationUsecase::new(Arc::clone(&pool));

    // get project version id
    let (_, project_version_list) = rawdata_usecase
        .list_project_versions(&ListProjectVersionRequest {
            product: "akxxx".into(),
            trial: "yyy".into(),
        })
        .await?;
    assert!(!project_version_list.is_empty());
    let version_id = project_version_list.first().unwrap().id;

    // create annotation version
    let target_annotation_version_name = "v1.0";
    let target_annotation_version_description = "Version 1.0";
    let reply = annotation_usecase
        .create_annotation_version(&CreateAnnotationVersionRequest {
            project_version_id: version_id,
            name: target_annotation_version_name.to_owned(),
            description: target_annotation_version_description.to_owned(),
        })
        .await?;
    let annotation_version = reply;
    assert_eq!(annotation_version.name, target_annotation_version_name);
    assert_eq!(
        annotation_version.description,
        target_annotation_version_description
    );

    // list annotation versions
    let annotation_versions = annotation_usecase
        .list_annotation_versions(version_id)
        .await?;
    assert!(!annotation_versions.is_empty());

    let annotation_version_id = annotation_version.id;

    // create form domain
    let form_list = rawdata_usecase
        .list_forms(&ListFormsRequest { version_id })
        .await?;
    assert!(!form_list.is_empty());
    let form_id = form_list.first().unwrap().id;

    let reply = annotation_usecase
        .create_form_domain(&CreateFormDomainRequest {
            annotation_version_id,
            form_id,
            name: "AE".to_owned(),
            description: "Adverse Event".to_owned(),
        })
        .await?;
    let form_domain = reply;
    assert_eq!(form_domain.name, "AE");
    assert_eq!(form_domain.description, "Adverse Event");

    // create annotation
    let reply = annotation_usecase
        .create_annotation(&CreateAnnotationRequest {
            annotation_version_id,
            form_id,
            variable: None,
            location: AnnotationLocation {
                source_id: 1,
                kind: AnnotationKind::Item,
            },
            annotation_display: "AETERM".into(),
            assign: false,
        })
        .await?;
    assert_eq!(reply.kind, AnnotationKind::Item);
    assert_eq!(reply.annotation_display, "AETERM");
    assert_eq!(reply.source_id, 1);

    // check if annotation list is not empty
    let list = annotation_usecase
        .list_annotation_by_form(&ListAnnotationByFormRequest {
            form_id,
            annotation_version_id,
        })
        .await?;
    assert_eq!(list.item.len(), 1);

    let variable_id = reply.variable_id;
    assert!(variable_id.eq(&-1));

    // update annotation (without changing variable)
    let reply = annotation_usecase
        .update_annotation(
            reply.id,
            &UpdateAnnootationRequest {
                variable: None,
                not_submit: false,
                annotation_display: "AETERM".into(),
                assign: true,
            },
        )
        .await?;
    assert!(reply.assign);
    assert_eq!(variable_id, reply.variable_id);

    // update annotation (changing variable)
    let reply = annotation_usecase
        .update_annotation(
            reply.id,
            &UpdateAnnootationRequest {
                variable: Some(VariableBinding {
                    domain_id: form_domain.id,
                    variable_name: "AETERM".into(),
                    supp: false,
                }),
                not_submit: false,
                annotation_display: "AETERM".into(),
                assign: true,
            },
        )
        .await?;
    assert!(reply.variable_id.ne(&-1));

    // remove form domain
    annotation_usecase
        .remove_form_domain(form_domain.id)
        .await?;

    // check if annotation list is empty
    let list = annotation_usecase
        .list_annotation_by_form(&ListAnnotationByFormRequest {
            form_id,
            annotation_version_id,
        })
        .await?;
    assert_eq!(list.item.len(), 0);
    Ok(())
}
