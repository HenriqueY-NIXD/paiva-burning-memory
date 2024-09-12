use sqlx::types::chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Params {
    pub album: String,
    pub artist: String,
    pub listen_at: Option<NaiveDate>,
    pub order: Option<i64>,
    pub photo: Option<String>,
}

pub struct ParamsDb {
    pub album: String,
    pub artist_id: i32,
    pub listen_at: Option<NaiveDate>,
    pub order: Option<i64>,
    pub photo: Option<String>,
}