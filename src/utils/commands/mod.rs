use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::Error;

mod ping;
mod help;
mod get;
mod post;
mod delete;
mod edit;

pub async fn cmd_ping(ctx: &Context, msg: &Message) -> Result<(), Error> {
    if let Err(err) = ping::ping(ctx, msg).await {
        msg.channel_id.say(&ctx.http, "Erro ao executar comand!").await?;
        println!("Erro ao Utilizar comando \"é\" (\"ping\"): {}", err);
    }

    Ok(())
}

pub async fn cmd_help(ctx: &Context, msg: &Message) -> Result<(), Error> {
    if let Err(err) = help::help(ctx, msg).await {
        msg.channel_id.say(&ctx.http, "Erro ao executar comand!").await?;
        println!("Erro ao Utilizar comando \"help\" (\"help\"): {}", err);
    }

    Ok(())
}

pub async fn cmd_get(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if let Err(err) = get::get(ctx, msg, args).await {
        msg.channel_id.say(&ctx.http, "Erro ao executar comand!").await?;
        println!("Erro ao Utilizar comando \"get\": {}", err);
    }

    Ok(())
}

pub async fn cmd_get_all(ctx: &Context, msg: &Message) -> Result<(), Error> {
    if let Err(err) = get::get_all(ctx, msg).await {
        msg.channel_id.say(&ctx.http, "Erro ao executar comand!").await?;
        println!("Erro ao Utilizar comando \"list\": {}", err);
    }

    Ok(())
}

pub async fn cmd_get_by_artist(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if let Err(err) = get::get_by_artist(ctx, msg, args).await {
        msg.channel_id.say(&ctx.http, "Erro ao executar comand!").await?;
        println!("Erro ao Utilizar comando \"artist\": {}", err);
    }

    Ok(())
}

pub async fn cmd_post(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if let Err(err) = post::post(ctx, msg, args).await {
        msg.channel_id.say(&ctx.http, "Erro ao executar comand!").await?;
        println!("Erro ao Utilizar comando \"post\": {}", err);
    }

    Ok(())
}

pub async fn cmd_delete(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if let Err(err) = delete::delete(ctx, msg, args).await {
        msg.channel_id.say(&ctx.http, "Erro ao executar comand!").await?;
        println!("Erro ao Utilizar comando \"delete\": {}", err);
    }

    Ok(())
}

pub async fn cmd_edit(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Error> {
    if let Err(err) = edit::edit(ctx, msg, args).await {
        msg.channel_id.say(&ctx.http, "Erro ao executar comand!").await?;
        println!("Erro ao Utilizar comando \"edit\": {}", err);
    }

    Ok(())
}