use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

#[group]
#[commands(ping)]
struct General;

struct Handler;

impl EventHandler for Handler {}

const BOT_TOKEN: &str = "NzAzMjMzNjI3MTI3MDIxNjU5.XqLosQ.V-UMWsxOCPhUyZrKofrqBqowOaA";

fn main() {
    // Login with a bot token from the environment
    let mut client =
        Client::new(BOT_TOKEN, Handler).expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("echo ")) // set the bot's prefix to "~"
            .group(&GENERAL_GROUP),
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
