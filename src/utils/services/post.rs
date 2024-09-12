use serenity::prelude::Context;

use crate::utils::database::post::create_artist as create_artist_db;

pub async fn create_artist(artist: &str, ctx: &Context) -> Result<i32, sqlx::Error> {
    create_artist_db(artist, ctx).await
}
