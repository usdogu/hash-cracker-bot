use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use serenity::client::bridge::gateway::{ShardManager};
use serenity::framework::standard::macros::{command, group, help};
use serenity::framework::standard::{
    help_commands, Args, CommandGroup,  CommandResult, HelpOptions,
    StandardFramework,
};

use serenity::model::channel::{Message};
use serenity::model::gateway::{GatewayIntents};
use serenity::model::id::UserId;
use serenity::prelude::*;
use tokio::sync::Mutex;
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}


#[group]
#[commands(info, hash)]
struct General;

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

pub(crate) async fn start() {
    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .prefix("!")
                // In this case, if "," would be first, a message would never
                // be delimited at ", ", forcing you to trim your arguments if you
                // want to avoid whitespaces at the start of each.
                .delimiters(vec![", ", ","])
            
        })
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::all();
    let token = std::env::var("DISCORD_TOKEN").unwrap();
    let mut client = Client::builder(token, intents)
        //.event_handler(Handler)
        .framework(framework)
        .type_map_insert::<CommandCounter>(HashMap::default())
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
#[description = "show info."]
async fn info(context: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &context.http,
            "Crack SHA{1,256,512}, MYSQL{3,5}, MD5 hashes.\nBy: <@581523213004046360>",
        )
        .await?;
    Ok(())
}

#[command]
#[description = "handle a hash."]
async fn hash(context: &Context, msg: &Message, args: Args) -> CommandResult {
    let hash = args.parse::<String>().unwrap();
    debug!("Cracking hash {}",hash);
    let cracked_hash = crate::check_hash(&hash).await;
    debug!("Hash {} cracked: {}",hash,cracked_hash);
    msg.channel_id.say(&context.http,format!("{hash}: {cracked_hash}")).await?;
    Ok(())
}
