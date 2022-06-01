use crate::check_hash;
use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommands};

pub(crate) async fn start() {
    let bot = Bot::from_env()
    .auto_send();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a hash.")]
    Hash(String),
    #[command(description = "show info.")]
    Info,
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Hash(hash) => {
            if hash.is_empty() {
                bot.send_message(
                    message.chat.id,
                    format!("Usage: '/hash <hash>'"),
                )
                .await?;
                return Ok(());
            };
            debug!("Cracking hash in TG {}",hash);
            let cracked_hash = check_hash(&hash).await;
            debug!("Hash {} cracked: {}",hash,cracked_hash);
            bot.send_message(message.chat.id, format!("{hash}: {cracked_hash}"))    
            .await?
        }
        Command::Info => {
            bot.send_message(message.chat.id,"Crack SHA{1,256,512}, MYSQL{3,5}, MD5 hashes.\nBy: @c25dbb82028af8907c3a402fa0c2780d")
            .await?
        },
    };

    Ok(())
}
