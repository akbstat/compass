use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct History {
    pub id: Option<i32>,
    pub product: String,
    pub trial: String,
    pub purpose: String,
}

// #[derive(Debug, Deserialize, IntoParams)]
// pub struct ListHistoriesRequest {
//     /// user id from aactive directory service of akeso
//     pub user: String,
// }

#[derive(Debug, Serialize, ToSchema)]
pub struct ListHistoriesReply {
    pub data: Vec<History>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SaveHistoryRequest {
    /// product name, for example: "ak101"
    pub product: String,
    /// trial name, for example: "101"
    pub trial: String,
    /// purpose name, for example: "dryrun"
    pub purpose: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SaveHistoryReply {
    pub data: Vec<History>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RemoveHistoriesRequest {
    /// user history id
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RemoveHistoriesReply {}
