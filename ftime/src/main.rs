use std::process;

use chrono::{DateTime, Local};
use ctrlc;
use owo_colors::OwoColorize;

const FERRIS: &str = r#"
    _~^~^~_
\) /  o o  \ (/
  '_   -   _'
  / '-----' \
"#;

const TIME_X_OFFSET: i32 = 5;
const TIME_Y_OFFSET: i32 = 1;

fn main() {
    ctrlc::set_handler(move || {
        print!("{}", " ".default_color().on_default_color());
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    clear();
    hide_cursor();

    movexy(0, TIME_Y_OFFSET); // place for time
    print_ferris(); // display Ferris only once

    loop {
        movexy(TIME_X_OFFSET, TIME_Y_OFFSET);
        print_time();
        std::thread::sleep(std::time::Duration::from_millis(999));
    }
}

fn print_time() {
    let local: DateTime<Local> = Local::now();
    println!("{}", local.format("%H:%M:%S").blue());
}

fn print_ferris() {
    print!("{}", FERRIS.yellow());
}

fn clear() {
    print!("\x1b[2J");
}

fn hide_cursor() {
    print!("\x1b[?25l");
}

fn movexy(x: i32, y: i32) {
    print!("\x1b[{y};{x}H");
}
