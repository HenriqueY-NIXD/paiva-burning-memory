use serenity::all::{
    Color, ComponentInteractionDataKind, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption, EditMessage,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::time::Duration;

use serenity::Error;

use crate::utils::docs::list_command;
use crate::utils::services;
use crate::utils::types::{str_to_command_category, CommandCategory, HelpInfoType};

use serenity::futures::StreamExt;

pub async fn help(ctx: &Context, msg: &Message) -> Result<(), Error> {
    let infos: HelpInfoType = list_command();

    let mut commands: String = String::new();
    let mut options: Vec<CreateSelectMenuOption> = Vec::new();
    let mut categories: Vec<&CommandCategory> = Vec::new();

    for info in infos.clone().into_iter() {
        let mut command = String::new();

        command.push_str(
            format!(
                "**`{}`**  ☛  {}\n",
                info.command_name, info.command_instructions
            )
            .as_str(),
        );

        commands.push_str(command.as_str());

        for category in info.command_categories {
            if categories.contains(&category) {
                continue;
            }

            categories.push(category);
            options.push(CreateSelectMenuOption::new(
                category.as_str(),
                category.as_enum_str(),
            ));
        }
    }

    let mut description = format!("**Prefix:  `paiva!`**\n\n{}", commands);

    if description.len() > 4096 {
        description = String::from("Muitos comandos para exibir nesta mensagem");
    }

    let avatar = services::get_avatar_from_bot(ctx);

    let embed = CreateEmbed::new()
        .title("Commands")
        .author(CreateEmbedAuthor::new(&ctx.cache.current_user().name).icon_url(&avatar))
        .description(description)
        .color(Color::from_rgb(50, 168, 82))
        .footer(CreateEmbedFooter::new("Selecione a categoria de comandos"));

    let builder = CreateMessage::new().embed(embed).select_menu(
        CreateSelectMenu::new(
            "category_select",
            CreateSelectMenuKind::String {
                options: options.clone(),
            },
        )
        .custom_id("category_select")
        .placeholder("Selecione uma categoria de comandos."),
    );

    let mut mensage_sent = msg
        .channel_id
        .send_message(&ctx.http, builder)
        .await
        .unwrap();

    let mut interaction_stream = mensage_sent
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(30))
        .stream();

    let mut category: CommandCategory;

    while let Some(interaction) = interaction_stream.next().await {
        category = match &interaction.data.kind {
            ComponentInteractionDataKind::StringSelect { values } => {
                match str_to_command_category(&values[0]) {
                    Ok(data) => data,
                    Err(_) => return Ok(()),
                }
            }
            _ => panic!("unexpected interaction data kind"),
        };

        commands = String::new();

        for info in infos.clone().into_iter() {
            if !info.command_categories.contains(&category) {
                continue;
            };

            let mut command = String::new();

            let mut aliases = String::new();

            for (index, alias) in info.command_alias.iter().enumerate() {
                aliases.push_str(alias);

                if index + 1 != info.command_alias.len() {
                    aliases.push(',');
                }
            }

            command.push_str(
                format!(
                    "**`{}`**  ☛  {}\nAliases:{}\nUsage: {}\n\n",
                    info.command_name, info.command_instructions, aliases, info.command_usage
                )
                .as_str(),
            );

            commands.push_str(command.as_str());
        }

        let mut description = format!("**Prefix:  `paiva!`**\n\n{}", commands);

        if description.len() > 4096 {
            description = String::from("Muitos comandos para exibir nesta mensagem");
        }

        let embed = CreateEmbed::new()
            .title("Commands")
            .author(CreateEmbedAuthor::new(&ctx.cache.current_user().name).icon_url(&avatar))
            .description(description)
            .color(Color::from_rgb(50, 168, 82))
            .footer(CreateEmbedFooter::new("Selecione a categoria de comandos"));

        let builder = CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .embed(embed)
                .select_menu(
                    CreateSelectMenu::new(
                        "category_select",
                        CreateSelectMenuKind::String {
                            options: options.clone(),
                        },
                    )
                    .custom_id("category_select")
                    .placeholder(category.as_enum_str()),
                ),
        );

        interaction.create_response(&ctx, builder).await?;
    }

    mensage_sent
        .edit(
            &ctx,
            EditMessage::new().select_menu(
                CreateSelectMenu::new(
                    "category_select",
                    CreateSelectMenuKind::String {
                        options: options.clone(),
                    },
                )
                .disabled(true)
                .placeholder("Timeout!"),
            ),
        )
        .await
        .unwrap();

    Ok(())
}
