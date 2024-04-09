use serenity::all::{CommandOptionType, CreateAttachment, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use std::path::Path;

async fn create_attachment(value: String /*impl Future<Output = Result<CreateAttachment, Error>>*/) -> String {
    let p = CreateAttachment::path(value).await.unwrap();
    serde_json::to_string(&p).unwrap()
}

pub fn run(options: &[ResolvedOption], now_path: &mut String) -> String {
    if let Some(ResolvedOption { value: _String, .. }) = options.first() {
        let file = format!("{:?}", options[0].value)
                .trim_start_matches("String(\"")
                .trim_end_matches("\")")
                .to_string();
        let timed_path = format!("{now_path}\\{file}");
        if Path::new(&timed_path).exists() {
            futures::executor::block_on(create_attachment(timed_path))
        } else {
            "Not valid file name".to_string()
        }
    } else {
        "Not valid options".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("sendfile")
        .description("Sending file")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "file name", "file name for sending")
                .required(true),
        )
}