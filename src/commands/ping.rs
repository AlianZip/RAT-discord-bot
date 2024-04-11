use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run() -> String {
    "pong!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping_bot").description("pong!")
}
