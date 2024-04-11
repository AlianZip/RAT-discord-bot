use serenity::builder::CreateCommand;

pub fn run(now_path: &mut String) -> String {
    now_path.to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pwd").description("print now path")
}
