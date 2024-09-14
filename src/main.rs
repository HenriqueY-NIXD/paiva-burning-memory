use anyhow::Context as _;
use std::sync::Arc;

use tokio::sync::RwLock;
use url::Url;

use sqlx::postgres::PgPoolOptions;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::{Client, Context, EventHandler, GatewayIntents};

mod utils;

use utils::types::CooldownContainer;
use utils::{commands, types::PgPoolContainer};

use shuttle_runtime::SecretStore;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with("paiva!") {
            return;
        }

        let args: Vec<&str> = msg.content.split(' ').collect();

        if args.len() <= 1 {
            return;
        }

        let _ = match args[1].to_lowercase().as_str() {
            "é" => commands::cmd_ping(&ctx, &msg).await,
            "help" | "commands" => commands::cmd_help(&ctx, &msg).await,
            "get" | "pega" | "take" => commands::cmd_get(&ctx, &msg, args).await,
            "getall" | "pegatudo" | "tudo" | "all" | "list" | "pega_tudo" | "take_all"
            | "takeall" => commands::cmd_get_all(&ctx, &msg).await,
            "artist" | "band" | "banda" | "artista" => {
                commands::cmd_get_by_artist(&ctx, &msg, args).await
            }
            "post" | "add" | "adicionar" | "adiciona" | "bota" | "coloca" | "new" => {
                commands::cmd_post(&ctx, &msg, args).await
            }
            "delete" | "remove" | "tira" | "pop" | "remover" | "deletar" | "cut" | "take_off"
            | "takeoff" => commands::cmd_delete(&ctx, &msg, args).await,
            "edit" | "editar" | "put" | "patch" | "atualizar" | "update" => {
                commands::cmd_edit(&ctx, &msg, args).await
            }
            "dia" | "day" | "dai" => {
                commands::cmd_today(&ctx, &msg).await
            }
            _ => {
                return;
            }
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;

        let pass = secret_store.get("PASS").context("'PASS' was not found")?;
        let user = secret_store.get("USER").context("'USER' was not found")?;
        let host = secret_store.get("HOST").context("'HOST' was not found")?;
        let database = secret_store
            .get("DATABASE")
            .context("'DATABASE' was not found")?;
        let port = secret_store.get("PORT").context("'PORT' was not found")?;

        let db_uri = format!("postgres://{}:{}/{}", host, port, database);

        let mut uri = Url::parse(&db_uri).unwrap();
        let _ = uri.set_username(user.as_str());
        let _ = uri.set_password(Some(pass.as_str()));
        let uri = uri.as_str();

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(uri)
            .await
            .expect("Não foi possível a conexão com o banco");

        data.insert::<PgPoolContainer>(pool);
        data.insert::<CooldownContainer>(Arc::new(RwLock::new(Vec::new())));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    Ok(client.into())
}
