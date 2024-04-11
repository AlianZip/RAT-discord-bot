use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use std::path::Path;

pub fn run(options: &[ResolvedOption], now_path: &mut String) -> String {
    if let Some(ResolvedOption { value: _String, .. }) = options.first() {
        let file = format!("{:?}", options[0].value)
            .trim_start_matches("String(\"")
            .trim_end_matches("\")")
            .to_string();
        let timed_path = format!("{now_path}\\{file}");
        if Path::new(&timed_path).exists() {
            timed_path
        } else {
            "not".to_string()
        }
    } else {
        "not".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("sendfile")
        .description("Sending file")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "filename",
                "file name for sending",
            )
            .required(true),
        )
}
