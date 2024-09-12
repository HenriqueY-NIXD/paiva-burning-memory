use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::Error as SerenityError;
use sqlx::types::chrono::NaiveDate;

use crate::utils::database::{edit, get};
use crate::utils::types::edit::Params;

// paiva! edit <id> <column> <value>
pub async fn edit(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), SerenityError> {
    if args.len() <= 4 {
        msg.reply(
            &ctx.http,
            "Digite todos os parâmetros necessários (**id, nome da coluna, valor**), ex:\n\
            **`paiva! edit 1 name \"Ultra Ego\"`**",
        )
        .await?;
        return Ok(());
    }

    let params = get_parameters(args);

    match params {
        Ok(data) => {
            if let Some(order) = data.order {
                if (get::get_by_id(ctx, order).await).is_ok() {
                    msg.reply(ctx, "Já existe uma música nesta posição!.")
                        .await?;
                    return Ok(());
                }
            }

            match edit::edit(ctx, data).await {
                Err(err) => {
                    msg.reply(ctx, "Erro ao atualizar musica.").await?;
                    println!("Erro em PUT: {}", err);
                }
                Ok(_) => {
                    msg.reply(ctx, "Musica atualizada!").await?;
                }
            }
        }
        Err(err) => {
            msg.reply(
                &ctx.http,
                format!("**{}** Digite todos os parâmetros necessários (id, nome da coluna, valor), ex:\n\
                    **`paiva! edit 1 name \"Ultra Ego\"`**\n\nColunas: **`album`**, **`artist`**, **`listen_at`**, **`order`** e **`photo`**. \
                    Mais nomes de colunas podem ser verificadas no comando `**help**`.", err)
                    .to_string(),
            )
            .await?;
            return Ok(());
        }
    }

    Ok(())
}

fn get_parameters(args: Vec<&str>) -> Result<Params, String> {
    let args_clone = args.clone().split_off(2);

    let args_value = args_clone.clone().split_off(2);
    let args_formatted = args_value.join(" ").trim().to_string();

    let id: i64;
    let mut album = None;
    let mut artist = None;
    let mut listen_at = None;
    let mut order = None;
    let mut photo = None;

    if let Ok(data) = args_clone[0].parse::<i64>() {
        id = data;
    } else {
        return Err(String::from(
            "Parâmetro (id/order) inválido, necessita ser um número existente na lista!",
        ));
    }

    match args_clone[1] {
        "album" | "albo" | "name" | "nome" => {
            album = Some(args_formatted);
        }
        "artist" | "band" | "artista" | "banda" | "rapper" => {
            artist = Some(args_formatted);
        }
        "listenat" | "listen_at" | "ouvido" | "ouvido_em" | "ouvidoem" | "listen" | "date"
        | "data" => {
            let format;

            if args_formatted.contains('/') {
                let split: Vec<&str> = args_formatted.split('/').collect();

                if split[0].len() == 2 {
                    format = String::from("%d/%m/%Y");
                } else if split[0].len() == 4 {
                    format = String::from("%Y/%m/%d");
                } else {
                    return Err(String::from(
                        "Parâmetro (listen_at) com formato desconhecido",
                    ));
                }
            } else if args_formatted.contains('-') {
                let split: Vec<&str> = args_formatted.split('-').collect();

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
            match NaiveDate::parse_from_str(args_clone[1], format.as_str()) {
                Ok(data) => {
                    listen_at = Some(data);
                }
                Err(_) => {
                    return Err(String::from("Parâmetro (listen_at) inválido"));
                }
            }
        }
        "order" | "id" | "ordem" | "posição" | "posicao" | "position" | "identificador" => {
            if let Ok(data) = args_formatted.parse::<i64>() {
                order = Some(data);
            } else {
                return Err(String::from(
                    "O Valor de \"Order\" necessita ser um número válido!",
                ));
            }
        }
        "photo" | "foto" | "picture" => {
            photo = Some(args_formatted);
        }
        _ => {
            return Err(String::from("Coluna não encontrada!"));
        }
    }

    Ok(Params {
        id,
        album,
        artist,
        listen_at,
        order,
        photo
    })
}
