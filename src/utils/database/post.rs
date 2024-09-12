use serenity::prelude::Context;

use crate::utils::types::post::ParamsDb;
use crate::utils::types::PgPoolContainer;

pub async fn post(ctx: &Context, params: ParamsDb) -> Result<i32, std::io::Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let mut query = "INSERT INTO album_listen(album, artist_id".to_string();

    let mut listen_param = String::new();
    let mut order_param = String::new();
    let mut photo_param = String::new();

    if params.listen_at.is_some() {
        query.push_str(", listen_at");
        listen_param = String::from(",$3");
    }

    if params.order.is_some() {
        query.push_str(", \"order\"");

        if params.listen_at.is_some() {
            order_param = String::from(",$4")
        } else {
            order_param = String::from(",$3")
        }
    }

    if params.photo.is_some() {
        query.push_str(", photo");

        if params.listen_at.is_some() && params.order.is_some() {
            photo_param = String::from(",$5")
        } else if params.listen_at.is_some() || params.order.is_some() {
            photo_param = String::from(",$4")
        } else {
            photo_param = String::from(",$3")
        }
    }

    query.push_str(
        format!(
            ") VALUES($1,$2{}{}{}) RETURNING id;",
            listen_param, order_param, photo_param
        )
        .as_str(),
    );

    let mut db_execution = sqlx::query_as(query.as_str())
        .bind(params.album)
        .bind(params.artist_id);

    if let Some(data) = params.listen_at {
        db_execution = db_execution.bind(data);
    }

    if let Some(data) = params.order {
        db_execution = db_execution.bind(data);
    }

    if let Some(data) = params.photo {
        db_execution = db_execution.bind(data);
    }

    let id: (i32,) = db_execution.fetch_one(&db).await.unwrap();

    let data: (i32,) = sqlx::query_as("SELECT \"order\" FROM album_listen WHERE id = $1")
        .bind(id.0)
        .fetch_one(&db)
        .await
        .unwrap();

    Ok(data.0)
}

pub async fn create_artist(artist: &str, ctx: &Context) -> Result<i32, sqlx::Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let row: (i32,) = sqlx::query_as("INSERT INTO artist(\"name\") VALUES($1) RETURNING id;")
        .bind(artist)
        .fetch_one(&db)
        .await
        .unwrap();

    Ok(row.0)
}
