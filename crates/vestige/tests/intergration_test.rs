use apis::vestige::History;
use dotenv::dotenv;
use sqlx::PgPool;
use std::{env, error::Error, sync::Arc};
use vestige::VestigeUsecase;

#[tokio::test]
async fn vestige_integration_test() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = Arc::new(PgPool::connect(&database_url).await?);
    let uc = VestigeUsecase::new(pool);
    let username = "yuqi.chen";

    // find projects
    let histories = uc.list_histories(username).await?;
    assert_eq!(histories.len(), 0);

    // simulate visit a new project
    let histories = uc
        .save_history(
            username,
            &History {
                id: None,
                product: "ak101".into(),
                trial: "101".into(),
                purpose: "draft".into(),
            },
        )
        .await?;
    assert_eq!(histories.len(), 1);

    // simulate visit a old project
    let histories = uc
        .save_history(
            username,
            &History {
                id: None,
                product: "ak101".into(),
                trial: "101".into(),
                purpose: "draft".into(),
            },
        )
        .await?;
    assert_eq!(histories.len(), 1);

    // remove projects
    let history_id = vec![histories[0].id.unwrap()];
    uc.remove_histories(&history_id).await?;

    let histories = uc.list_histories(username).await?;
    assert_eq!(histories.len(), 0);

    Ok(())
}
