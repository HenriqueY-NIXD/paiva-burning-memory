use serenity::model::channel::Message;
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
                msg.channel_id.say(&ctx.http, "Amanhã é o ULTIMO DIA!!! hoje é dia 364".to_string()).await?;    
                return Ok(());
            }

            if data > 365 {
                msg.channel_id.say(&ctx.http, "O desafio foi completado! Parabéns!!!".to_string()).await?;    
                return Ok(());
            }

            let first_date = if let Some(x) = NaiveDate::from_ymd_opt(2023, 11, 23) {x} else { panic!() };
            let first_date_plus_db = if let Some(x) = first_date.checked_add_days(Days::new(data as u64))  {x} else { panic!() };
            let today = Local::now().date_naive();
            let diff = today.signed_duration_since(first_date_plus_db);

            if first_date_plus_db > today {
                msg.channel_id.say(&ctx.http, format!("Dia **{}**, falta **{}** e esta devendo **{}** dias", data-1, (364 - data), diff)).await?;
                return Ok(());
            }

            msg.channel_id.say(&ctx.http, format!("Dia **{}**, falta **{}** e esta devendo **{}** dias", data, (365 - data), diff)).await?;
        }
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Erro ao puxar o último dia!").await?;
        }
    };

    Ok(())
}