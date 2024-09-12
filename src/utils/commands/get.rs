use std::time::Duration;
use rand::{thread_rng, Rng};
use serenity::all::{
    parse_emoji, Color, CreateButton, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, EditMessage,
};
use serenity::futures::StreamExt;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::Error;

use crate::utils::database::get;
use crate::utils::services;
use crate::utils::types::commands::CommandsEnum;

pub async fn get(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if args.len() <= 2 {
        msg.reply(
            &ctx.http,
            "Digite o album que deseja pesquisar, ex:\n`paiva! get id 1`\n\nOu por nome:\n`paiva! get name Ultra Ego`",
        )
        .await?;
        return Ok(());
    }

    match args[2].to_lowercase().as_str().trim() {
        "id" | "order" | "ordem" | "identificador" => get_by_id(ctx, msg, args).await,
        "name" | "nome" | "album" => get_by_name(ctx, msg, args).await,
        _ => Ok(()),
    }
}

pub async fn get_by_id(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if args.len() <= 3 {
        msg.reply(
            &ctx.http,
            "Digite o id/ordem do album que deseja pesquisar, ex:\n`paiva! get id 1`",
        )
        .await?;
        return Ok(());
    }

    let avatar = services::get_avatar_from_bot(ctx);

    if let Ok(id) = args[3].parse::<i64>() {
        match get::get_by_id(ctx, id).await {
            Ok(data) => {
                let mut embed = CreateEmbed::new()
                    .title(format!("**{}**", data.0))
                    .author(
                        CreateEmbedAuthor::new(format!("{}º Album da lista", id)).icon_url(&avatar),
                    )
                    .description(format!("**Artist:** {}", data.1))
                    .color(Color::from_rgb(50, 168, 82));

                match data.3 {
                    Some(photo) => {
                        embed = embed.thumbnail(photo);
                    },
                    None => {
                        embed = embed.thumbnail("https://aamsreremiodemmzpfdl.supabase.co/storage/v1/object/public/IMGS/WHERE.png");
                    }
                }

                if let Some(footer_info) = data.2 {
                    embed = embed.footer(CreateEmbedFooter::new(format!(
                        "Ouvido em: {}",
                        footer_info
                    )));
                }

                let builder = CreateMessage::new().embed(embed);

                msg.channel_id.send_message(&ctx.http, builder).await?;
            }
            Err(_) => {
                msg.reply(&ctx.http, format!("Não achei este album... (**{}º**)", id))
                    .await?;
            }
        };
    } else {
        msg.reply(
            &ctx.http,
            "Parâmetro inválido.\nDigite o album que deseja pesquisar, ex:\n```paiva! get 1```",
        )
        .await?;
    }

    Ok(())
}

pub async fn get_by_name(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if args.len() <= 3 {
        msg.reply(
            &ctx.http,
            "Digite o album que deseja pesquisar, ex:\n`paiva! get name Ultra Ego`",
        )
        .await?;
        return Ok(());
    }

    let avatar = services::get_avatar_from_bot(ctx);

    let mut args_clone = args.clone();
    args_clone = args_clone.split_off(3);
    let args_formatted = args_clone.join(" ").trim().to_string();

    match get::get_by_album(ctx, String::from(&args_formatted)).await {
        Ok(data) => {
            let mut embed = CreateEmbed::new()
                .title(format!("**{}**", data.0))
                .author(
                    CreateEmbedAuthor::new(format!("{}º Album da lista", data.3)).icon_url(&avatar),
                )
                .description(format!("**Artist:** {}", data.1))
                .color(Color::from_rgb(50, 168, 82));

            if let Some(footer_info) = data.2 {
                embed = embed.footer(CreateEmbedFooter::new(format!(
                    "Ouvido em: {}",
                    footer_info
                )));
            }

            let builder = CreateMessage::new().embed(embed);

            msg.channel_id.send_message(&ctx.http, builder).await?;
        }
        Err(_) => match get::get_by_album_like(ctx, String::from(&args_formatted)).await {
            Ok(data) => {
                if data.is_empty() {
                    let embed = CreateEmbed::new()
                        .title(format!(
                            "Não encontrei \"{}\" em específico",
                            &args_formatted
                        ))
                        .author(CreateEmbedAuthor::new("Não encontrado...").icon_url(&avatar))
                        .color(Color::from_rgb(50, 168, 82));

                    let builder = CreateMessage::new().embed(embed);

                    msg.channel_id.send_message(&ctx.http, builder).await?;

                    return Ok(());
                }

                if data.len() == 1 {
                    let mut embed = CreateEmbed::new()
                        .title(format!("**{}**", data[0].0))
                        .author(
                            CreateEmbedAuthor::new(format!("{}º Album da lista", data[0].2))
                                .icon_url(&avatar),
                        )
                        .description(format!("**Artist:** {}", data[0].1))
                        .color(Color::from_rgb(50, 168, 82));

                    if let Some(footer_info) = data[0].3 {
                        embed = embed.footer(CreateEmbedFooter::new(format!(
                            "Ouvido em: {}",
                            footer_info
                        )));
                    }

                    let builder = CreateMessage::new().embed(embed);

                    msg.channel_id.send_message(&ctx.http, builder).await?;
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

pub async fn get_all(ctx: &Context, msg: &Message) -> Result<(), Error> {
    if !(services::check_cooldown(ctx, msg, CommandsEnum::GetAll).await) {
        return Ok(());
    }

    match get::get_all(ctx).await {
        Ok(data) => {
            let avatar = services::get_avatar_from_bot(ctx);

            let mut descriptions: Vec<String> = Vec::new();

            let mut index = 0;

            let mut description = String::new();

            let length = data.len();

            for (index_album, album) in data.into_iter().enumerate() {
                if index > 19 {
                    descriptions.push(description.clone());
                    description = String::new();
                    index = 0;
                }

                description
                    .push_str(format!("**{}.** `{}` - {}\n", album.3, album.0, album.1).as_str());

                if length == index_album + 1 {
                    descriptions.push(description.clone());
                }

                index += 1;
            }

            if descriptions.is_empty() {
                msg.channel_id
                    .say(&ctx.http, "Não tem albuns cadastrados.".to_string())
                    .await?;
                return Ok(());
            }

            let mut pagination = 0;

            let description = &descriptions[pagination];

            let embed = CreateEmbed::new()
                .author(
                    CreateEmbedAuthor::new(format!(
                        "Albuns da lista (Pág: {}/{})",
                        pagination + 1,
                        descriptions.len()
                    ))
                    .icon_url(&avatar),
                )
                .description(description)
                .color(Color::from_rgb(50, 168, 82))
                .footer(CreateEmbedFooter::new(
                    "Aperte os botões para mudar o índice da lista.",
                ));

            let mut builder = CreateMessage::new().embed(embed);

            if descriptions.len() > 1 {
                builder = builder
                    .button(
                        CreateButton::new("first")
                            .emoji(parse_emoji("<:b_previous_double:848383585807106064>").unwrap()),
                    )
                    .button(
                        CreateButton::new("previous")
                            .emoji(parse_emoji("<:b_previous:848383585962819585>").unwrap()),
                    )
                    .button(
                        CreateButton::new("next")
                            .emoji(parse_emoji("<:b_next:848383585374830623>").unwrap()),
                    )
                    .button(
                        CreateButton::new("last")
                            .emoji(parse_emoji("<:b_next_double:848383585701330944>").unwrap()),
                    );
            }

            let mut mensage_sent = msg.channel_id.send_message(&ctx.http, builder).await?;

            let mut interaction_stream = mensage_sent
                .await_component_interaction(&ctx.shard)
                .timeout(Duration::from_secs(30))
                .stream();

            while let Some(interaction) = interaction_stream.next().await {
                let rule = interaction.data.custom_id.as_str();

                match rule {
                    "first" => {
                        pagination = 0;
                    }
                    "previous" => {
                        pagination = pagination.saturating_sub(1);
                    }
                    "next" => {
                        if pagination != descriptions.len() - 1 {
                            pagination += 1;
                        }
                    }
                    "last" => {
                        pagination = descriptions.len() - 1;
                    }
                    _ => (),
                }

                let description = &descriptions[pagination];

                let embed = CreateEmbed::new()
                    .author(
                        CreateEmbedAuthor::new(format!(
                            "Albuns da lista (Pág: {}/{})",
                            pagination + 1,
                            descriptions.len()
                        ))
                        .icon_url(&avatar),
                    )
                    .description(description)
                    .color(Color::from_rgb(50, 168, 82))
                    .footer(CreateEmbedFooter::new(
                        "Aperte os botões para mudar o índice da lista.",
                    ));

                let builder =
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .embed(embed)
                            .button(CreateButton::new("first").emoji(
                                parse_emoji("<:b_previous_double:848383585807106064>").unwrap(),
                            ))
                            .button(
                                CreateButton::new("previous").emoji(
                                    parse_emoji("<:b_previous:848383585962819585>").unwrap(),
                                ),
                            )
                            .button(
                                CreateButton::new("next")
                                    .emoji(parse_emoji("<:b_next:848383585374830623>").unwrap()),
                            )
                            .button(CreateButton::new("last").emoji(
                                parse_emoji("<:b_next_double:848383585701330944>").unwrap(),
                            )),
                    );

                interaction.create_response(&ctx, builder).await?;
            }

            if descriptions.len() <= 1 {
                return Ok(());
            }

            mensage_sent
                .edit(
                    &ctx,
                    EditMessage::new()
                        .button(
                            CreateButton::new("first")
                                .emoji(
                                    parse_emoji("<:b_previous_double:848383585807106064>").unwrap(),
                                )
                                .disabled(true),
                        )
                        .button(
                            CreateButton::new("previous")
                                .emoji(parse_emoji("<:b_previous:848383585962819585>").unwrap())
                                .disabled(true),
                        )
                        .button(
                            CreateButton::new("next")
                                .emoji(parse_emoji("<:b_next:848383585374830623>").unwrap())
                                .disabled(true),
                        )
                        .button(
                            CreateButton::new("last")
                                .emoji(parse_emoji("<:b_next_double:848383585701330944>").unwrap())
                                .disabled(true),
                        ),
                )
                .await
                .unwrap();
        }
        Err(err) => {
            println!("Erro em get_all: {}", err);
            msg.channel_id
                .say(&ctx.http, "Não tem albuns cadastrados.".to_string())
                .await?;
        }
    };

    Ok(())
}

pub async fn get_by_artist(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if !(services::check_cooldown(ctx, msg, CommandsEnum::Artist).await) {
        return Ok(());
    }

    if args.len() <= 2 {
        msg.reply(
            &ctx.http,
            "Digite o artista que deseja pesquisar, ex:\n```paiva! artist Kanye East```",
        )
        .await?;
        return Ok(());
    }

    let avatar = services::get_avatar_from_bot(ctx);

    let mut args_clone = args.clone();
    args_clone = args_clone.split_off(2);
    let args_formatted = args_clone.join(" ").trim().to_string();

    match get::get_artist_by_artist(ctx, String::from(&args_formatted)).await {
        Ok(data) => {
            return show_artist_albums(ctx, msg, data.0, avatar).await;
        }
        Err(_) => match get::get_artist_by_artist_like(ctx, String::from(&args_formatted)).await {
            Ok(data) => {
                if data.is_empty() {
                    let embed = CreateEmbed::new()
                        .title(format!(
                            "Não encontrei \"{}\" em específico",
                            &args_formatted
                        ))
                        .author(CreateEmbedAuthor::new("Não encontrado...").icon_url(&avatar))
                        .color(Color::from_rgb(50, 168, 82));

                    let builder = CreateMessage::new().embed(embed);

                    msg.channel_id.send_message(&ctx.http, builder).await?;

                    return Ok(());
                }

                if data.len() == 1 {
                    return show_artist_albums(ctx, msg, data[0].0.clone(), avatar).await;
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
                        "Não encontrei \"{}\" em específico nos albums registrados",
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

async fn show_artist_albums(
    ctx: &Context,
    msg: &Message,
    artist: String,
    avatar: String,
) -> Result<(), Error> {
    let mut descriptions: Vec<String> = Vec::new();
    let mut photos: Vec<String> = Vec::new();
    let mut artist_id: i32 = -1;

    match get::get_by_artist(ctx, artist.clone()).await {
        Ok(data) => {
            artist_id = data[0].1;
            let length = data.len();
            let mut index = 0;
            let mut description = String::new();

            for (index_album, album) in data.into_iter().enumerate() {
                if index > 19 {
                    descriptions.push(description.clone());
                    description = String::new();
                    index = 0;
                }

                description.push_str(format!("**{}.** `{}`\n", index_album + 1, album.0).as_str());

                if let Some(data) = album.2 {
                    photos.push(data);
                }

                if length == index_album + 1 {
                    descriptions.push(description.clone());
                }

                index += 1;
            }
        }
        Err(_) => {
            descriptions.push(String::from("Nenhum album com este artista..."));
        }
    }

    let mut photo: String = String::from("https://aamsreremiodemmzpfdl.supabase.co/storage/v1/object/public/IMGS/WHERE.png");

    if !photos.is_empty() {
        let index = thread_rng().gen_range(0..(photos.len()- 1));

        photo = photos[index].clone();
    }

    if artist_id != -1 {
        if let Ok(data) = get::get_artist_by_id(ctx, artist_id).await {
            if let Some(photo_db) = data.1 {
                photo = photo_db;
            }
        }
    }

    let mut pagination = 0;

    let description = &descriptions[pagination];

    let embed = CreateEmbed::new()
        .title(format!("**{}**", artist))
        .author(
            CreateEmbedAuthor::new(format!(
                "Albuns do artista (Pág: {}/{})",
                pagination + 1,
                descriptions.len()
            ))
            .icon_url(&avatar),
        )
        .description(description)
        .color(Color::from_rgb(50, 168, 82))
        .thumbnail(photo);

    let mut builder = CreateMessage::new().embed(embed.clone());

    if descriptions.len() > 1 {
        builder = builder
            .embed(embed.footer(CreateEmbedFooter::new(
                "Aperte os botões para mudar o índice da lista.",
            )))
            .button(
                CreateButton::new("first")
                    .emoji(parse_emoji("<:b_previous_double:848383585807106064>").unwrap()),
            )
            .button(
                CreateButton::new("previous")
                    .emoji(parse_emoji("<:b_previous:848383585962819585>").unwrap()),
            )
            .button(
                CreateButton::new("next")
                    .emoji(parse_emoji("<:b_next:848383585374830623>").unwrap()),
            )
            .button(
                CreateButton::new("last")
                    .emoji(parse_emoji("<:b_next_double:848383585701330944>").unwrap()),
            );
    }

    let mut mensage_sent = msg.channel_id.send_message(&ctx.http, builder).await?;

    if descriptions.len() <= 1 {
        return Ok(())
    }

    let mut interaction_stream = mensage_sent
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(30))
        .stream();

    while let Some(interaction) = interaction_stream.next().await {
        let rule = interaction.data.custom_id.as_str();

        match rule {
            "first" => {
                pagination = 0;
            }
            "previous" => {
                pagination = pagination.saturating_sub(1);
            }
            "next" => {
                if pagination != descriptions.len() {
                    pagination += 1;
                }
            }
            "last" => {
                pagination = descriptions.len() - 1;
            }
            _ => (),
        }

        let description = &descriptions[pagination];

        let embed = CreateEmbed::new()
            .author(
                CreateEmbedAuthor::new(format!(
                    "Albuns da lista (Pág: {}/{})",
                    pagination + 1,
                    descriptions.len()
                ))
                .icon_url(&avatar),
            )
            .description(description)
            .color(Color::from_rgb(50, 168, 82))
            .footer(CreateEmbedFooter::new(
                "Aperte os botões para mudar o índice da lista.",
            ));

        let builder = CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .embed(embed)
                .button(
                    CreateButton::new("first")
                        .emoji(parse_emoji("<:b_previous_double:848383585807106064>").unwrap()),
                )
                .button(
                    CreateButton::new("previous")
                        .emoji(parse_emoji("<:b_previous:848383585962819585>").unwrap()),
                )
                .button(
                    CreateButton::new("next")
                        .emoji(parse_emoji("<:b_next:848383585374830623>").unwrap()),
                )
                .button(
                    CreateButton::new("last")
                        .emoji(parse_emoji("<:b_next_double:848383585701330944>").unwrap()),
                ),
        );

        interaction.create_response(&ctx, builder).await?;
    }

    if descriptions.len() <= 1 {
        return Ok(());
    }

    mensage_sent
        .edit(
            &ctx,
            EditMessage::new()
                .button(
                    CreateButton::new("first")
                        .emoji(parse_emoji("<:b_previous_double:848383585807106064>").unwrap())
                        .disabled(true),
                )
                .button(
                    CreateButton::new("previous")
                        .emoji(parse_emoji("<:b_previous:848383585962819585>").unwrap())
                        .disabled(true),
                )
                .button(
                    CreateButton::new("next")
                        .emoji(parse_emoji("<:b_next:848383585374830623>").unwrap())
                        .disabled(true),
                )
                .button(
                    CreateButton::new("last")
                        .emoji(parse_emoji("<:b_next_double:848383585701330944>").unwrap())
                        .disabled(true),
                ),
        )
        .await
        .unwrap();

    Ok(())
}