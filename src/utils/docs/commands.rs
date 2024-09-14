use crate::utils::types::{HelpInfo, CommandCategory,HelpInfoType};

pub const COMMANDS: HelpInfoType = [ 
    HelpInfo {
        command_name: "é",
        command_alias: &[],
        command_instructions: "Comando para pingar bot, e xingar o Paiva de trivela.",
        command_usage: "paiva! é",
        command_categories: &[CommandCategory::General],
    },
    HelpInfo {
        command_name: "dia",
        command_alias: &["dai", "day"],
        command_instructions: "Comando para saber as estatísticas.",
        command_usage: "paiva! dia",
        command_categories: &[CommandCategory::PaivaBurningMemory],
    },
    HelpInfo {
        command_name: "help",
        command_alias: &["commands"],
        command_instructions: "Comando para listar os comandos do BOT.",
        command_usage: "paiva! help",
        command_categories: &[CommandCategory::General],
    },
    HelpInfo {
        command_name: "get",
        command_alias: &["pega", "take"],
        command_instructions: "Comando para Visualizar album da lista por nome ou por numero da lista.\nAliases para \"album\": \
            `name`, `nome`;\nAliases para \"order\": `id`, `ordem`, `identificador`.",
        command_usage: "paiva! get album Ultra Ego   |   paiva! get order 1",
        command_categories: &[CommandCategory::PaivaBurningMemory],
    },
    HelpInfo {
        command_name: "list",
        command_alias: &["getall", "pegatudo", "tudo", "all", "list", "pega_tudo", "take_all"],
        command_instructions: "Comando para listar todos os albuns da lista. (Cooldown de 30s por user)",
        command_usage: "paiva! list",
        command_categories: &[CommandCategory::PaivaBurningMemory],
    },
    HelpInfo {
        command_name: "artist",
        command_alias: &["band", "banda", "artista"],
        command_instructions: "Comando para listar todos os albuns da lista de um artista. (Cooldown de 30s por user)",
        command_usage: "paiva! artist Feed Me Jack",
        command_categories: &[CommandCategory::PaivaBurningMemory],
    },
    HelpInfo {
        command_name: "add",
        command_alias: &["post", "adicionar", "adiciona", "bota", "coloca", "new"],
        command_instructions: "Comando para cadastrar album na lista. Argumentos listen_at, order e photo são opcionais.",
        command_usage: "\n`paiva! add \"Ultra Ego\" \"Feed Me Jack\"`   |   \n`paiva! add \"Ultra Ego\" \"Feed Me Jack\" 01/01/2024 1 https://photo_here.com`",
        command_categories: &[CommandCategory::PaivaBurningMemory],
    },
    HelpInfo {
        command_name: "delete",
        command_alias: &["remove", "tira", "pop", "remover", "deletar", "cut", "take_off", "takeoff"],
        command_instructions: "Comando para remover album na lista por nome ou id.\nAliases para \"album\": \
            `name`, `nome`;\nAliases para \"order\": `id`, `ordem`, `identificador`.",
        command_usage: "paiva! delete id 1   |   paiva! delete name Ultra Ego",
        command_categories: &[CommandCategory::PaivaBurningMemory],
    },
    HelpInfo {
        command_name: "edit",
        command_alias: &["editar", "put", "patch", "atualizar", "update"],
        command_instructions: "Comando para editar album na lista por id e nome da coluna.\n\nColunas:\n\
            - **name** (album, albo, nome);\n- **artist** (band, artista, banda, rapper);\n\
            - **listen_at** (listenat, listen_at, ouvido, ouvido_em, ouvidoem, listen, date, data);\n\
            - **order** (id, ordem, posição, posicao, position, identificador);\n\
            - **photo** (foto, picture).",
        command_usage: "paiva! edit <id> <name|artist|listen_at|order|photo> <value> -> paiva! edit 1 name \"Ultra Ego\"",
        command_categories: &[CommandCategory::PaivaBurningMemory],
    },
];