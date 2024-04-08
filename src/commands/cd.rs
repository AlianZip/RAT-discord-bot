use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(options: &[ResolvedOption], now_path: &mut String) -> String {
    if let Some(ResolvedOption { value: _String, .. }) = options.first() {
        let dir = format!("{:?}", options[0].value)
            .trim_start_matches("String(\"")
            .trim_end_matches("\")")
            .to_string();
        now_path.insert_str(now_path.len(), &format!("\\{}",&dir));
        dir
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
