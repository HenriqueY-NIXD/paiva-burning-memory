use sqlx::types::chrono::NaiveDate;

#[derive(Debug)]
pub struct Params {
    pub id: i64,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub listen_at: Option<NaiveDate>,
    pub order: Option<i64>,
    pub photo: Option<String>
}