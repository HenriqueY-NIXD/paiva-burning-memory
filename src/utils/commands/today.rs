use serenity::all::{Color, CreateEmbedFooter, CreateMessage};
use serenity::{all::CreateEmbed, model::channel::Message};
use serenity::prelude::Context;
use serenity::Error as SerenityError;
use sqlx::types::chrono::{NaiveDate, Local};
use chrono::Days;

use crate::utils::database::get;

pub async fn today(ctx: &Context, msg: &Message) -> Result<(), SerenityError> {
    get_last_order(ctx, msg).await?;

    Ok(())
}

pub async fn get_last_order(ctx: &Context, msg: &Message) -> Result<(), SerenityError> {
    match get::get_last_order(ctx).await {
        Ok(data) => {
            if data == 365 {
                msg.channel_id.say(&ctx.http, "Amanhã é o ULTIMO DIA!!! hoje é dia 366".to_string()).await?;    
                return Ok(());
            }

            if data > 365 {
                msg.channel_id.say(&ctx.http, "**O desafio foi completado! Parabéns!!!**".to_string()).await?;
                let embed = CreateEmbed::new()
                    .title("FINALMENTE PORRA!".to_string())
                    .thumbnail("https://aamsreremiodemmzpfdl.supabase.co/storage/v1/object/public/IMGS/god.png")
                    .description("Depois de 366 dias...".to_string())
                    .color(Color::from_rgb(50, 168, 82))
                    .footer(CreateEmbedFooter::new("Crazy? I was crazy once, they made me listen to 366 albuns for a year.".to_string()));
                
                let builder = CreateMessage::new().embed(embed);

                msg.channel_id.send_message(&ctx.http, builder).await?; 
                return Ok(());
            }
            
            let first_date = if let Some(x) = NaiveDate::from_ymd_opt(2023, 11, 23) {x} else { panic!() };
            let first_date_plus_db = if let Some(x) = first_date.checked_add_days(Days::new((data - 1) as u64))  {x} else { panic!() };
            let today = Local::now().date_naive();
            let diff = today.signed_duration_since(first_date_plus_db);

            if first_date_plus_db > today {
                msg.channel_id.say(&ctx.http, format!("Dia **{}**, falta **{}** e esta devendo **{}** dias", data-1, (366 - data), diff.num_days())).await?;
                return Ok(());
            }

            msg.channel_id.say(&ctx.http, format!("Dia **{}**, falta **{}** e esta devendo **{}** dias", data, (366 - data), diff.num_days())).await?;
        }
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Erro ao puxar o último dia!").await?;
        }
    };

    Ok(())
}