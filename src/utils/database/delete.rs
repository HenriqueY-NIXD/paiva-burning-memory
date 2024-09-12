use serenity::prelude::Context;
use sqlx::Error;

use crate::utils::types::PgPoolContainer;

pub async fn delete(ctx: &Context, id: i64) -> Result<(), Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    sqlx::query("DELETE FROM album_listen WHERE \"order\" = $1".to_string().as_str())
        .bind(id)
        .execute(&db)
        .await?;

    Ok(())
}