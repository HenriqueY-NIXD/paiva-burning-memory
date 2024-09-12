use std::time::Duration;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use sqlx::types::chrono::Utc;

pub mod post;

use super::types::{commands::CommandsEnum, CooldownContainer, CooldownInfo};

pub fn get_avatar_from_bot(ctx: &Context) -> String {
    match ctx.cache.current_user().avatar_url() {
        None => String::from("https://assets.vogue.com/photos/5c14171e18854824cbc06e54/master/w_2560%2Cc_limit/00-story-image-the-caretaker-everywhere-at-the-end-of-time-leyland-kirby%2520(1).jpg"),
        Some(data) => data
    }
}

pub async fn check_cooldown(ctx: &Context, msg: &Message, command: CommandsEnum) -> bool {
    let cooldown = {
        let data = ctx.data.read().await;
        data.get::<CooldownContainer>().unwrap().clone()
    };

    let user_cooldown = {
        let cooldown = cooldown.read().await;
        cooldown
            .clone()
            .into_iter()
            .find(|obj| obj.user_id == msg.author.id.to_string() && obj.command == command)
    };

    match user_cooldown {
        Some(data) => {
            let now = Utc::now().naive_utc();
            let diference = now - data.used_at;
            
            if Duration::from_secs(diference.num_seconds().try_into().unwrap()) <= Duration::from_secs(30) {
                msg.reply(
                    &ctx.http,
                    format!("Cooldown de 30s neste comando, falta {}/30s", diference.num_seconds())
                )
                .await.unwrap();
                return false;
            }
        },
        None => {
            let mut counter = cooldown.write().await;
            counter.push(CooldownInfo {
                command,
                user_id: msg.author.id.to_string(),
                used_at: Utc::now().naive_utc()
            });
        }
    }

    true
}