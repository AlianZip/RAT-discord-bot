#![allow(non_snake_case)]

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serde_json;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Config {
    TOKEN: String,
}

#[allow(dead_code)]
fn config_reader() -> Config {
    let res = include_str!("config.json");
    let des: Config = serde_json::from_str(res).unwrap();
    return des;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token  = config_reader().TOKEN;
    
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.unwrap();

    // Start listening for events by starting a single shard
    if let Err(_) = client.start().await {
       todo!();
    }
}


