use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::Error as SerenityError;
use sqlx::types::chrono::NaiveDate;
use url::form_urlencoded;
use musicbrainz_rs::entity::artist::Artist;
use musicbrainz_rs::prelude::*;

use crate::utils::database::{get, post};
use crate::utils::services::post::create_artist;
use crate::utils::types::post::{Params, ParamsDb};

pub async fn post(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), SerenityError> {
    if args.len() <= 2 {
        msg.reply(
            &ctx.http,
            "Digite todos os parâmetros necessários (album, artist), ex:\n\
            **`paiva! add \"Ultra Ego\" \"Feed Me Jack\"`**\n\n\
            Parâmetros opcionais também estão disponíveis (listen_at, order, photo). ex:\n\
            **`paiva! add \"Ultra Ego\" \"Feed Me Jack\" 01/01/2024 1 https://photo_here.com`**",
        )
        .await?;
        return Ok(());
    }

    let params = get_parameters(args);

    match params {
        Ok(data) => {
            if let Some(order) = data.order {
                if let Ok(exist) = get::get_by_id(ctx, order).await {
                    msg.reply(
                        ctx,
                        format!("Já existe uma musica nesta Ordem **`{}`**.", exist.0),
                    )
                    .await?;
                    return Ok(());
                }
            }

            let artist: i32 = match get::get_artist_id_by_artist(ctx, &data.artist).await {
                Ok(result) => result.0,
                Err(_) => create_artist(&data.artist, ctx).await.unwrap(),
            };

            let mut photo = data.photo.clone();

            // if data.photo.is_none() {
            //     let query = Artist::
            //     query_builder()
            //         .artist(data.artist.clone())
            //         .build();
            // }

            match post::post(ctx, ParamsDb {
                album: data.album,
                artist_id: artist,
                listen_at: data.listen_at,
                order: data.order,
                photo
            }).await {
                Err(err) => {
                    msg.reply(ctx, "Erro ao inserir musica.").await?;
                    println!("Erro em POST: {}", err);
                }
                Ok(data) => {
                    msg.reply(ctx, format!("Musica inserida na posição: **`{}º`**", data))
                        .await?;
                }
            }
        }
        Err(_) => {
            msg.reply(
                &ctx.http,
                "Digite todos os parâmetros necessários (album, artist), ex:\n\
                **`paiva! add \"Ultra Ego\" \"Feed Me Jack\"`**\n\n\
                Parâmetros opcionais também estão disponíveis (listen_at, order, photo). ex:\n\
                **`paiva! add \"Ultra Ego\" \"Feed Me Jack\" 01/01/2024 1 https://photo_here.com`**\n\
                Datas podem ter os seguintes formatos: \n\
                `yyyy-mm-dd`, `dd-mm-yyyy`, `yyyy/mm/dd` ou `dd/mm/yyyy`.".to_string(),
            )
            .await?;
            return Ok(());
        }
    }

    Ok(())
}

fn get_parameters(args: Vec<&str>) -> Result<Params, String> {
    let mut args_clone = args.clone();
    args_clone = args_clone.split_off(2);
    let args_formatted = args_clone.join(" ").trim().to_string();

    let mut parts: Vec<String> = Vec::new();
    let mut current_part = String::new();
    let mut in_quotes = false;

    for c in args_formatted.chars() {
        match c {
            '"' => {
                if in_quotes {
                    parts.push(current_part.clone());
                    current_part.clear();
                }
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                if !current_part.is_empty() {
                    parts.push(current_part.clone());
                    current_part.clear();
                }
            }
            _ => current_part.push(c),
        }
    }
    if !current_part.is_empty() {
        parts.push(current_part);
    }

    if parts.len() < 2 {
        return Err(String::from("Parâmetros inválidos!"));
    }

    let album = parts[0].clone();
    let artist = parts[1].clone();

    let mut listen_at = None;
    let mut order = None;
    let mut photo = None;

    if let Some(date_str) = parts.get(2) {
        let date_str = date_str.trim();
        if date_str.contains('/') || date_str.contains('-') {
            let listen_at_param = date_str.to_string();

            if listen_at_param.len() > 10 || listen_at_param.len() < 10 {
                return Err(String::from("Parâmetro (listen_at) inválido!"));
            }

            let format;

            if listen_at_param.contains('/') {
                let split: Vec<&str> = listen_at_param.split('/').collect();

                if split[0].len() == 2 {
                    format = String::from("%d/%m/%Y");
                } else if split[0].len() == 4 {
                    format = String::from("%Y/%m/%d");
                } else {
                    return Err(String::from(
                        "Parâmetro (listen_at) com formato desconhecido",
                    ));
                }
            } else if listen_at_param.contains('-') {
                let split: Vec<&str> = listen_at_param.split('-').collect();

                if split[0].len() == 2 {
                    format = String::from("%d-%m-%Y");
                } else if split[0].len() == 4 {
                    format = String::from("%Y-%m-%d");
                } else {
                    return Err(String::from(
                        "Parâmetro (listen_at) com formato desconhecido",
                    ));
                }
            } else {
                return Err(String::from(
                    "Parâmetro (listen_at) com formato desconhecido",
                ));
            }

            match NaiveDate::parse_from_str(listen_at_param.as_str(), format.as_str()) {
                Ok(data) => {
                    listen_at = Some(data);
                }
                Err(_) => {
                    return Err(String::from("Parâmetro (listen_at) inválido"));
                }
            }
        } else {
            match date_str.parse() {
                Ok(data) => {
                    listen_at = Some(data);
                }
                Err(_) => {
                    return Err(String::from("Parâmetro (listen_at) inválido"));
                }
            }
        }
    }

    if let Some(order_str) = parts.get(3) {
        let order_str = order_str.trim();

        match order_str.parse::<i64>() {
            Ok(data) => {
                order = Some(data);
            }
            Err(_) => {
                return Err(String::from("Parâmetro (order) inválido"));
            }
        }
    }

    if let Some(data) = parts.get(4) {
        photo = Some(data.to_string());
    }

    Ok(Params {
        album,
        artist,
        listen_at,
        order,
        photo,
    })
}
