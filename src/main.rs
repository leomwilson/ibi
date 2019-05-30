use colored::*;
use std::env;
use std::path::Path;

fn pwd() -> String {
    let home = env::var("HOME").unwrap(); // get the user's home directory
    let path = format!("{}", env::current_dir().unwrap_or("".into()).display()); // get the current path
    if path.starts_with(&home) {
        path.replacen(&home, "~", 1) // only replace the first instance
    }
    else {
        path
    }
}

fn venv() -> String {
    match env::var("VIRTUAL_ENV") {
        Ok(path) => {
            if let Some(name) = Path::new(&path).file_name() {
                if let Some(utf8_name) = name.to_str() { // .file_name() returns an OsString, we want a proper String (UTF-8 only)
                    return format!("[{}] ", utf8_name);
                }
            }
        },
        Err(_) => (),
    }
    return "".to_string();
}

fn main() {
    println!("{} {}", pwd().bright_blue(), venv().yellow().dimmed());
}
