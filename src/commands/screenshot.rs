use serenity::all::CreateCommand;

pub fn run() -> String {
    "sometext".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("screenshot").description("Do screenshot")
}
