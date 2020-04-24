use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework, CommandError
};
use serenity::http::client::Http;
use serenity::model::channel::Message;
use serenity::model::id::MessageId;
use serenity::prelude::{Context, EventHandler};

use std::env;

#[group]
#[commands(ping)]
struct General;

#[group]
#[prefix(message)]
#[commands(count)]
struct MessageGroup;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(
        &env::var("DISCORD_TOKEN").expect("Discord bot token"),
        Handler,
    )
    .expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("echo ")) // set the bot's prefix
            .group(&GENERAL_GROUP)
            .group(&MESSAGEGROUP_GROUP),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn count(ctx: &mut Context, msg: &Message) -> CommandResult {

    msg.reply(&ctx, "Counting messages")?;

    let snowflake: MessageId = msg.id;

    match ctx.http.get_messages(
        msg.channel_id.0,
        &format!("before={}", snowflake.to_string()),
    ) {
        Ok(messages) => println!("{:?}", messages),
        Err(e) => return Err(CommandError(e.to_string())),
    }

    Ok(())
}
