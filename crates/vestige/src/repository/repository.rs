use super::dao::{HistoryRow, UserHistoryRow, UserHistoryViewRow, UserRow};
use apis::vestige::History;
use chrono::Local;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct VestigeRepository {
    pool: Arc<sqlx::PgPool>,
}

impl VestigeRepository {
    pub fn new(pool: Arc<sqlx::PgPool>) -> VestigeRepository {
        VestigeRepository { pool }
    }

    pub async fn list_user_histories(
        &self,
        user: &str,
    ) -> Result<Vec<UserHistoryViewRow>, sqlx::Error> {
        let user = self.get_or_create_user(user).await?;
        let rows: Vec<UserHistoryViewRow> = sqlx::query_as(
            r#"
SELECT id, product, trial, purpose 
FROM user_history_view
WHERE user_id = $1
ORDER BY updated_at
        "#,
        )
        .bind(user.id)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows)
    }

    pub async fn save_history(
        &self,
        user: &str,
        history: &History,
    ) -> Result<Vec<UserHistoryViewRow>, sqlx::Error> {
        let history = self.get_or_create_history(history).await?;
        let user = self.get_or_create_user(user).await?;
        self.create_or_update_user_history(user.id, history.id)
            .await?;
        self.list_user_histories(&user.name).await
    }

    pub async fn remove_user_histories(&self, ids: &[i32]) -> Result<(), sqlx::Error> {
        let mut syntax = "DELETE FROM user_history WHERE id in (".to_string();
        ids.iter().enumerate().for_each(|(index, _)| {
            if index.gt(&0) {
                syntax.push(',');
            }
            syntax.push_str(&format!("${}", index + 1));
        });
        syntax.push_str(")");
        let builder = sqlx::query(&syntax);
        let builder = ids.iter().fold(builder, |builder, id| builder.bind(id));
        builder.execute(self.pool.as_ref()).await?;
        Ok(())
    }

    async fn create_or_update_user_history(
        &self,
        user_id: i32,
        history_id: i32,
    ) -> Result<(), sqlx::Error> {
        let row: Option<UserHistoryRow> = sqlx::query_as(
            r#"
SELECT user_id, history_id, updated_at
FROM user_history
WHERE user_id = $1 AND history_id = $2
        "#,
        )
        .bind(user_id)
        .bind(history_id)
        .fetch_optional(self.pool.as_ref())
        .await?;
        match row {
            Some(mut row) => {
                row.updated_at = Local::now();
                self.update_user_history(&row).await
            }
            None => {
                let row = UserHistoryRow {
                    user_id,
                    history_id,
                    updated_at: Local::now(),
                };
                self.create_user_history(&row).await
            }
        }
    }

    async fn create_user_history(&self, row: &UserHistoryRow) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
INSERT INTO user_history (user_id, history_id, updated_at)
VALUES ($1, $2, $3)
        "#,
        )
        .bind(&row.user_id)
        .bind(&row.history_id)
        .bind(&row.updated_at)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    async fn update_user_history(&self, row: &UserHistoryRow) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
UPDATE user_history
SET updated_at = $1
WHERE user_id = $2 AND history_id = $3
        "#,
        )
        .bind(&row.updated_at)
        .bind(&row.user_id)
        .bind(&row.history_id)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    async fn get_or_create_user(&self, user: &str) -> Result<UserRow, sqlx::Error> {
        let row: Option<UserRow> = sqlx::query_as(
            r#"
SELECT id, name 
FROM compass_user 
WHERE name = $1
        "#,
        )
        .bind(user)
        .fetch_optional(self.pool.as_ref())
        .await?;
        match row {
            Some(row) => Ok(row),
            None => self.create_user(user).await,
        }
    }

    async fn create_user(&self, user: &str) -> Result<UserRow, sqlx::Error> {
        let row: UserRow = sqlx::query_as(
            r#"
INSERT INTO compass_user (name)
VALUES ($1) 
RETURNING id, name
"#,
        )
        .bind(user)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    async fn get_or_create_history(&self, history: &History) -> Result<HistoryRow, sqlx::Error> {
        let row: Option<HistoryRow> = sqlx::query_as(
            r#"
SELECT id, product, trial, purpose 
FROM history 
WHERE product = $1 AND trial = $2 AND purpose = $3
"#,
        )
        .bind(&history.product)
        .bind(&history.trial)
        .bind(&history.purpose)
        .fetch_optional(self.pool.as_ref())
        .await?;
        match row {
            Some(row) => Ok(row),
            None => self.create_history(history).await,
        }
    }

    async fn create_history(&self, history: &History) -> Result<HistoryRow, sqlx::Error> {
        let row: HistoryRow = sqlx::query_as(
            r#"
INSERT INTO history (product, trial, purpose)
VALUES ($1, $2, $3) 
RETURNING id, product, trial, purpose
"#,
        )
        .bind(&history.product)
        .bind(&history.trial)
        .bind(&history.purpose)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }
}
