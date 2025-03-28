use chrono::Local;
use std::process::exit;

use crate::help;
use crate::stop_watch;

pub enum Command {
    Enter,
    Help,
    StopWatch,
    Timer,
    Quit,
    Unknown(String),
}

#[allow(dead_code)]
fn get_now_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

pub fn parse(arg: &str) -> Command {
    match arg {
        "" => Command::Enter,
        "help" => Command::Help,
        "stopwatch" => Command::StopWatch,
        "timer" => Command::Timer,
        "quit" => Command::Quit,
        _ => Command::Unknown(arg.to_string()),
    }
}

pub fn run_command(cmd: Command) {
    match cmd {
        Command::Enter => {}
        Command::Help => help(),
        Command::StopWatch => stop_watch::stop_watch(),
        Command::Timer => timer(),
        Command::Quit => exit(0),
        Command::Unknown(cmd) => println!("command not found: {}", cmd),
    }
}

fn timer() {}
