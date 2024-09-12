use serenity::all::{Color, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::Error;

use crate::utils::database::delete;
use crate::utils::database::get;
use crate::utils::services;

pub async fn delete(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if args.len() <= 2 {
        msg.reply(
            &ctx.http,
            "Digite o Número do album (ou o nome do album), ex:\n\
            **`paiva! delete id 1`**\n\n\
            Pode ser usado o nome também, ex:\n\
            **`paiva! delete name Ultra Ego`**",
        )
        .await?;
        return Ok(());
    }

    match args[3].to_lowercase().as_str().trim() {
        "id" | "order" | "ordem" | "identificador" => delete_by_id(ctx, msg, args).await,
        "name" | "nome" => delete_by_name(ctx, msg, args).await,
        _ => Ok(()),
    }
}

pub async fn delete_by_name(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if args.len() <= 3 {
        msg.reply(
            &ctx.http,
            "Digite o nome do album que deseja deletar, ex:\n`paiva! delete name Ultra Ego`",
        )
        .await?;
        return Ok(());
    }

    let mut args_clone = args.clone();
    args_clone = args_clone.split_off(2);
    let args_formatted = args_clone.join(" ").trim().to_string();

    let avatar = services::get_avatar_from_bot(ctx);

    match get::get_by_album(ctx, String::from(&args_formatted)).await {
        Ok(data) => match delete::delete(ctx, data.3).await {
            Ok(_) => {
                msg.reply(
                    &ctx.http,
                    format!(
                        "Album **`{}`** da posição **`{}`** apagado.",
                        data.0, data.3
                    ),
                )
                .await?;
            }
            Err(err) => {
                msg.reply(&ctx.http, "Não foi possível apagar este album...")
                    .await?;
                println!("Erro em DELETE: {}", err);
                return Ok(());
            }
        },
        Err(_) => match get::get_by_album_like(ctx, String::from(&args_formatted)).await {
            Ok(data) => {
                if data.is_empty() {
                    msg.reply(&ctx.http, format!(
                        "Não encontrei \"{}\" em específico",
                        &args_formatted
                    )).await?;

                    return Ok(());
                }

                if data.len() == 1 {
                    match delete::delete(ctx, data[0].2.into()).await {
                        Ok(_) => {
                            msg.reply(
                                &ctx.http,
                                format!(
                                    "Album **`{}`** da posição **`{}`** apagado.",
                                    data[0].0, data[0].2
                                ),
                            )
                            .await?;
                        }
                        Err(err) => {
                            msg.reply(&ctx.http, "Não foi possível apagar este album...")
                                .await?;
                            println!("Erro em DELETE: {}", err);
                            return Ok(());
                        }
                    }
                    return Ok(());
                }

                let mut description = String::new();

                for (index, info) in data.into_iter().enumerate() {
                    description.push_str(format!("**{}.**`{}`\n", index + 1, info.0).as_str());
                }

                let embed = CreateEmbed::new()
                    .title(format!(
                        "Não encontrei \"{}\" em específico, você quis dizer?",
                        &args_formatted
                    ))
                    .author(CreateEmbedAuthor::new("Não encontrado...").icon_url(&avatar))
                    .description(description)
                    .color(Color::from_rgb(50, 168, 82))
                    .footer(CreateEmbedFooter::new(
                        "Tente denovo, talvez com algumas das opções acima.",
                    ));

                let builder = CreateMessage::new().embed(embed);

                msg.channel_id.send_message(&ctx.http, builder).await?;
            }
            Err(_) => {
                let embed = CreateEmbed::new()
                    .title(format!(
                        "Não encontrei \"{}\" em específico",
                        &args_formatted
                    ))
                    .author(CreateEmbedAuthor::new("Não encontrado...").icon_url(&avatar))
                    .color(Color::from_rgb(50, 168, 82));

                let builder = CreateMessage::new().embed(embed);

                msg.channel_id.send_message(&ctx.http, builder).await?;
            }
        },
    };

    Ok(())
}

pub async fn delete_by_id(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if args.len() <= 3 {
        msg.reply(
            &ctx.http,
            "Digite o id/ordem do album que deseja deletar, ex:\n`paiva! delete id 1`",
        )
        .await?;
        return Ok(());
    }

    if let Ok(id) = args[2].parse::<i64>() {
        match get::get_by_id(ctx, id).await {
            Ok(data) => match delete::delete(ctx, id).await {
                Ok(_) => {
                    msg.reply(
                        &ctx.http,
                        format!("Album **`{}`** da posição **`{}`** apagado.", data.0, id),
                    )
                    .await?;
                }
                Err(err) => {
                    msg.reply(&ctx.http, "Não foi possível apagar este album...")
                        .await?;
                    println!("Erro em DELETE: {}", err);
                    return Ok(());
                }
            },
            Err(_) => {
                msg.reply(&ctx.http, format!("Não achei este album... (**{}º**)", id))
                    .await?;
                return Ok(());
            }
        }
    } else {
        msg.reply(
            &ctx.http,
            "Parâmetro inválido.\nDigite o album que deseja deletar, ex:\n```paiva! delete id 1```",
        )
        .await?;
    }

    Ok(())
}