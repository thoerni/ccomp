pub static TERMINAL_RED: &str = "\u{001b}[31m";
pub static TERMINAL_GREEN: &str = "\u{001b}[32m";
pub static TERMINAL_BOLD: &str = "\033[31m";
pub static TERMINAL_STOP: &str = "\u{001b}[0m";


pub fn status(message: &str) {
    println!(
        "{bold}{green}➜{stop} {bold}{message}{stop}",
        message = message,
        bold = TERMINAL_BOLD,
        green = TERMINAL_GREEN,
        stop = TERMINAL_STOP
    );
}

pub fn error(message: &str) {
    eprintln!(
        " {bold}{red}➜{stop} {bold}{message}{stop}",
        message = message,
        bold = TERMINAL_BOLD,
        red = TERMINAL_RED,
        stop = TERMINAL_STOP
    );

}