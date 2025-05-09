use apis::vestige::History;
use chrono::{DateTime, Local};
use sqlx::prelude::FromRow;

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct HistoryRow {
    pub id: i32,
    pub product: String,
    pub trial: String,
    pub purpose: String,
}

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct UserHistoryViewRow {
    pub id: i32,
    pub product: String,
    pub trial: String,
    pub purpose: String,
}

#[derive(Debug, FromRow)]
pub struct UserHistoryRow {
    pub user_id: i32,
    pub history_id: i32,
    pub updated_at: DateTime<Local>,
}

impl Into<History> for UserHistoryViewRow {
    fn into(self) -> History {
        let UserHistoryViewRow {
            id,
            product,
            trial,
            purpose,
        } = self;
        History {
            id: Some(id),
            product,
            trial,
            purpose,
        }
    }
}
