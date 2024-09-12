use serenity::prelude::Context;

use crate::utils::types::edit::Params;
use crate::utils::types::PgPoolContainer;

pub async fn edit(ctx: &Context, params: Params) -> Result<(), std::io::Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let mut query = "UPDATE album_listen SET ".to_string();

    if params.album.is_some() {
        query.push_str("album = $1");
    }
    else if params.artist.is_some() {
        query.push_str("artist = $1");
    }
    else if params.listen_at.is_some() {
        query.push_str("listen_at = $1");
    }
    else if params.order.is_some() {
        query.push_str("\"order\" = $1");
    }

    query.push_str(" WHERE \"order\" = $2;");

    let mut db_execution = sqlx::query(query.as_str());

    if let Some(data) = params.album {
        db_execution = db_execution.bind(data);
    }

    if let Some(data) = params.artist {
        db_execution = db_execution.bind(data);
    }

    if let Some(data) = params.listen_at {
        db_execution = db_execution.bind(data);
    }

    if let Some(data) = params.order {
        db_execution = db_execution.bind(data);
    } 

    db_execution.bind(params.id).execute(&db).await.unwrap();

    Ok(())
}