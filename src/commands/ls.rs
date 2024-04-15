use serenity::builder::CreateCommand;
use std::fs::{metadata, read_dir};

pub fn run(now_path: &mut String) -> String {
    let mut files = String::new();
    let paths = read_dir(now_path.clone()).unwrap();
    for path in paths {
        let patht = format!("{:?}", path.unwrap().path().file_name().unwrap());

        if metadata(format!(
            "{}\\{}",
            now_path,
            patht
                .clone()
                .trim_start_matches("\"")
                .trim_end_matches("\"")
        ))
        .unwrap()
        .is_file()
        {
            files.insert_str(
                files.len(),
                &format!(
                    "{}\n",
                    patht.trim_start_matches("\"").trim_end_matches("\"")
                ),
            )
        } else if metadata(format!(
            "{}\\{}",
            now_path,
            patht
                .clone()
                .trim_start_matches("\"")
                .trim_end_matches("\"")
        ))
        .unwrap()
        .is_dir()
        {
            files.insert_str(
                files.len(),
                &format!(
                    "```{}```\n",
                    patht.trim_start_matches("\"").trim_end_matches("\"")
                ),
            )
        }
    }
    files
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ls").description("List of files and dirs")
}
