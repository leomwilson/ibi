use colored::*;
use std::env;
use std::path::Path;
mod vcs;

fn pwd() -> colored::ColoredString {
    let home = env::var("HOME").unwrap(); // get the user's home directory
    let mut path = format!("{}", env::current_dir().unwrap_or("".into()).display()); // get the current path
    if path.starts_with(&home) {
        path = path.replacen(&home, "~", 1); // only replace the first instance
    }
    return path.cyan()
}

fn venv() -> colored::ColoredString {
    match env::var("VIRTUAL_ENV") {
        Ok(path) => {
            if let Some(name) = Path::new(&path).file_name() {
                if let Some(utf8_name) = name.to_str() { // .file_name() returns an OsString, we want a proper String (UTF-8 only)
                    return format!("[{}] ", utf8_name).yellow().dimmed();
                }
            }
        },
        Err(_) => (),
    }
    return "".white();
}

fn pchar() -> colored::ColoredString {
    if env::var("HOME").unwrap() == "/root" {
        "#".red().bold()
    }
    else {
        "$".bright_magenta()
    }
}

fn main() {
    println!("{} {}{}\n{} ", pwd(), venv(), vcs::git(), pchar());
}
