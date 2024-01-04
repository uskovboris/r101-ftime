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

/// X coordinate for time indication
const TIME_X_OFFSET: u32 = 5;

/// Y coordinate for time indication
const TIME_Y_OFFSET: u32 = 1;

fn main() {
    // Remove program artefact from the screen and exit correct
    ctrlc::set_handler(move || {
        clear();
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

/// Prints time in HH:MM:SS format
/// 
/// # Usage
/// 
/// ```rust
/// print_time();
/// ```
/// 
/// # Example
/// 09:59:17
fn print_time() {
    let local: DateTime<Local> = Local::now();
    println!("{}", local.format("%H:%M:%S").blue());
}

/// Prints Ferris crab on the screen
/// 
/// # Usage
/// 
/// ```rust
/// print_ferris();
/// ```
fn print_ferris() {
    print!("{}", FERRIS.yellow());
}

/// Clears the screen
/// 
/// # Usage
/// 
/// ```rust
/// clear();
/// ```
fn clear() {
    print!("\x1b[2J");
}

/// Hides cursor
/// 
/// # Usage
/// 
/// ```rust
/// hide_cursor();
/// ```
fn hide_cursor() {
    print!("\x1b[?25l");
}

/// Moves cursor to point (x,y)
/// # Params
/// `x` column
/// 'y' row
/// 
/// # Usage
/// 
/// ```rust
/// movexy(10, 15);
/// ```
fn movexy(x: u32, y: u32) {
    print!("\x1b[{y};{x}H");
}
