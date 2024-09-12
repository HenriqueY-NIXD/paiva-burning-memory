use serenity::prelude::Context;
use sqlx::Error;

use sqlx::types::chrono::NaiveDate;

use crate::utils::types::PgPoolContainer;

pub async fn get_by_album(ctx: &Context, name: String) -> Result<(String, String, Option<NaiveDate>, i64), Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let row: (String, String, Option<NaiveDate>, i64) = 
        sqlx::query_as("
            SELECT al.album, a.\"name\", al.listen_at, al.\"order\" FROM album_listen al \
                JOIN artist a ON (a.id=al.artist_id) \
            WHERE LOWER(al.album) like LOWER($1);\
        ")
            .bind(name)
            .fetch_one(&db)
            .await?;

    Ok(row)
}

pub async fn get_by_artist(ctx: &Context, name: String) -> Result<Vec<(String, i32, Option<String>,)>, Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let row: Vec<(String, i32, Option<String>,)> = 
        sqlx::query_as("
            SELECT al.album, a.id artist_id, al.photo FROM album_listen al \
                JOIN artist a ON (a.id=al.artist_id) \
            WHERE LOWER(a.\"name\") like LOWER($1) ORDER BY al.\"order\";\
        ")
            .bind(name)
            .fetch_all(&db)
            .await?;

    Ok(row)
}

pub async fn get_artist_by_artist(ctx: &Context, name: String) -> Result<(String,), Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let row: (String,) = 
        sqlx::query_as("SELECT DISTINCT \"name\" FROM artist WHERE LOWER(\"name\") like LOWER($1);")
            .bind(name)
            .fetch_one(&db)
            .await?;

    Ok(row)
}

pub async fn get_artist_id_by_artist(ctx: &Context, name: &str) -> Result<(i32,), Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let row: (i32,) = 
        sqlx::query_as("SELECT id FROM artist WHERE LOWER(\"name\") like LOWER($1);")
            .bind(name)
            .fetch_one(&db)
            .await?;

    Ok(row)
}

pub async fn get_artist_by_artist_like(ctx: &Context, name: String) -> Result<Vec<(String,)>, Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let row: Vec<(String,)> = 
        sqlx::query_as(format!("SELECT DISTINCT \"name\" FROM artist WHERE LOWER(\"name\") like '%{}%' LIMIT 5;", name.to_lowercase()).as_str())
            .fetch_all(&db)
            .await.unwrap();

    Ok(row)
}

pub async fn get_by_album_like(ctx: &Context, name: String) -> Result<Vec<(String, String, i32, Option<NaiveDate>)>, Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let row: Vec<(String, String, i32, Option<NaiveDate>)> = 
        sqlx::query_as(format!("
            SELECT al.album, a.\"name\", al.\"order\", al.listen_at FROM album_listen al \
                JOIN artist a ON (al.artist_id=a.id) \
            WHERE LOWER(al.album) like '%{}%' LIMIT 5;\
        ", name.to_lowercase()).as_str()).fetch_all(&db).await.unwrap();

    Ok(row)
}

pub async fn get_by_id(ctx: &Context, id: i64) -> Result<(String, String, Option<NaiveDate>, Option<String>), Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let row: (String, String, Option<NaiveDate>, Option<String>) = 
        sqlx::query_as("
            SELECT al.album, a.\"name\", al.listen_at, al.photo FROM album_listen al \
                JOIN artist a ON (al.artist_id=a.id) \
            WHERE al.\"order\" = $1;\
        ").bind(id).fetch_one(&db).await.unwrap();

    Ok(row)
}

pub async fn get_artist_by_id(ctx: &Context, id: i32) -> Result<(String, Option<String>), Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let row: (String, Option<String>) = 
        sqlx::query_as("
            SELECT \"name\", photo FROM artist \
            WHERE id = $1;\
        ").bind(id).fetch_one(&db).await?;

    Ok(row)
}

pub async fn get_all(ctx: &Context) -> Result<Vec<(String, String, Option<NaiveDate>, i32)>, Error> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let row: Vec<(String, String, Option<NaiveDate>, i32)> = sqlx::query_as("
        SELECT al.album, a.\"name\", al.listen_at, al.\"order\" \
            JOIN artist a ON (al.artist_id=a.id) \
        FROM album_listen al ORDER BY al.\"order\";\
    ")
        .fetch_all(&db)
        .await?;

    Ok(row)
}