use std::sync::Arc;

use sqlx::PgPool;
use sqlx::types::chrono::{NaiveDate, NaiveDateTime};
use serenity::prelude::TypeMapKey;

use tokio::sync::RwLock;

pub mod post;
pub mod edit;
pub mod commands;

pub struct PgPoolContainer;

#[derive(Debug, Clone)]
pub struct CooldownContainer;

#[derive(Debug, Clone)]
pub struct CooldownInfo {
    pub user_id: String,
    pub command: commands::CommandsEnum,
    pub used_at: NaiveDateTime,
}

impl TypeMapKey for PgPoolContainer {
    type Value = PgPool;
}

impl TypeMapKey for CooldownContainer {
    type Value = Arc<RwLock<Vec<CooldownInfo>>>;
}

#[derive(Debug, Clone)]
pub struct HelpInfo {
    pub command_name: &'static str,
    pub command_alias: &'static [&'static str],
    pub command_instructions: &'static str,
    pub command_usage: &'static str,
    pub command_categories: &'static [CommandCategory],
}

pub type HelpInfoType = [HelpInfo; 9];

#[derive(Debug)]
pub struct AlbumListen {
    pub id: i32,
    pub album: String,
    pub artist: String,
    pub listen_at: NaiveDate,
    pub created_at: NaiveDate,
    pub order: i32,
}

#[derive(Debug, PartialEq)]
pub enum CommandCategory {
    General,
    PaivaBurningMemory
}

impl CommandCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            CommandCategory::General => "General",
            CommandCategory::PaivaBurningMemory => "Paiva's Burning Memory"
        }
    }

    pub fn as_enum_str(&self) -> &'static str {
        match self {
            CommandCategory::General => "General",
            CommandCategory::PaivaBurningMemory => "PaivaBurningMemory"
        }
    }
}

pub fn str_to_command_category(param: &str) -> Result<CommandCategory, std::io::Error> {
    match param {
        "General" => Ok(CommandCategory::General),
        "PaivaBurningMemory" => Ok(CommandCategory::PaivaBurningMemory),
        _ => panic!("This string does'nt exist in enum CommandCategory")
    }
}