use rand::{thread_rng, Rng};
use serenity::prelude::Context;
use serenity::model::channel::Message;

use serenity::Error;

const NICKNAMES: [&str; 11] = [
    "Zé ruéla",
    "Bostalho",
    "Zé",
    "Zé Mané",
    "Zé Ruela",
    "Peidão",
    "Frangão",
    "Energumino",
    "Otario",
    "Besta",
    "Pilantra"
];

pub async fn ping(ctx: &Context, msg: &Message) -> Result<(), Error> {
    let index = thread_rng().gen_range(0..(NICKNAMES.len() - 1));

    msg.reply(&ctx.http, NICKNAMES[index]).await?;
    Ok(())
}
