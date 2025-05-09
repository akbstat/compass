use crate::errors::Result;
use crate::repository::VestigeRepository;
use apis::vestige::History;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct VestigeUsecase {
    repo: VestigeRepository,
}

impl VestigeUsecase {
    pub fn new(pool: Arc<Pool<Postgres>>) -> VestigeUsecase {
        let repo = VestigeRepository::new(pool);
        VestigeUsecase { repo }
    }
    pub async fn list_histories(&self, user: &str) -> Result<Vec<History>> {
        let history = self.repo.list_user_histories(user).await?;
        Ok(history.into_iter().map(|h| h.into()).collect())
    }
    pub async fn save_history(&self, user: &str, history: &History) -> Result<Vec<History>> {
        let history = self.repo.save_history(user, history).await?;
        Ok(history.into_iter().map(|h| h.into()).collect())
    }
    pub async fn remove_histories(&self, id: &[i32]) -> Result<()> {
        self.repo.remove_user_histories(id).await?;
        Ok(())
    }
}
