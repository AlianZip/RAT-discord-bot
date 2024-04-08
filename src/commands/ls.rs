use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption], now_path: &mut String) -> String {
    now_path.to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ls").description("List of files and dirs")
}
