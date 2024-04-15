use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use webbrowser;

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption { value: _String, .. }) = options.first() {
        let link = format!("{:?}", options[0].value)
            .trim_start_matches("String(\"")
            .trim_end_matches("\")")
            .to_string();
        webbrowser::open(&link).expect("Err");
        "opening successfully".to_string()
    } else {
        "Not valid options".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("openurl")
        .description("open some url")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "url", "some url").required(true),
        )
}
