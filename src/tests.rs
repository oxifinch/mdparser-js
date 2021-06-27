use std::fs;
use std::path::Path;

pub fn read_markdown_file(file_path: &str) -> String {
    let file_path = Path::new(&file_path);
    let file_contents = match fs::read_to_string(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("[ ERROR ] Could not read file. Returning default markdown.");
            String::from("# Default markdown!")
        }
    };
    file_contents
}
