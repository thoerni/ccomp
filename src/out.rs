use colored::*;

pub fn status(message: &str) {
    let arrow = "➜".bold().green();
    let message = message.bold();
    println!("{} {}", arrow, message);
}

pub fn error(message: &str) {
    let arrow = "➜".bold().red();
    let message = message.bold();
    eprintln!(" {} {}", arrow, message);
}