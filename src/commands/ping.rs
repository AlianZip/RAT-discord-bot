use serenity::builder::CreateCommand;

pub fn run() -> String {
    "pong!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping_bot").description("pong!")
}
