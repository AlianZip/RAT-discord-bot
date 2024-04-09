use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use std::path::Path;

pub fn run(options: &[ResolvedOption], now_path: &mut String) -> String {
    if let Some(ResolvedOption { value: _String, .. }) = options.first() {
        let dir = format!("{:?}", options[0].value)
            .trim_start_matches("String(\"")
            .trim_end_matches("\")")
            .to_string();
        if dir == ".." {
            let last_slash = now_path.rfind('\\').unwrap();
            let last_str = &now_path[last_slash..];
            *now_path = now_path.trim_end_matches(last_str).to_string();
            return now_path.to_string();
        } else if dir == "." {
            return now_path.to_string();
        }
        let timed_path = format!("{now_path}\\{dir}");
        if Path::new(&timed_path).exists() {
            now_path.insert_str(now_path.len(), &format!("\\{}", &dir));
            now_path.to_string()
        } else {
            "Not valid path".to_string()
        }
    } else {
        "Not valid options".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("cd")
        .description("Change dir")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "dir", "Name of dir")
                .required(true),
        )
}
