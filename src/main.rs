use colored::*;
use std::env;

fn pwd() -> String {
    let home = env::var("HOME").unwrap(); // get the user's home directory
    let path = format!("{}", env::current_dir().unwrap_or("".into()).display()); // get the current path
    if path.starts_with(&home) {
        path.replace(&home, "~")
    }
    else {
        path
    }
}

fn main() {
    println!("{}", pwd().bright_blue());
}
