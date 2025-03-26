use chrono::Local;
use std::process::exit;

pub enum Command {
    Quit,
    StopWatch,
    Timer,
    Unknown(String),
}

fn get_now_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

pub fn parse(arg: &str) -> Command {
    match arg {
        "q" => Command::Quit,
        "st" => Command::StopWatch,
        "timer" => Command::Timer,
        _ => Command::Unknown(arg.to_string()),
    }
}

pub fn run_command(cmd: Command) {
    match cmd {
        Command::Quit => exit(0),
        Command::StopWatch => stop_watch(),
        Command::Timer => timer(),
        Command::Unknown(cmd) => println!("{}: command not found", cmd),
    }
}

fn stop_watch() {}

fn timer() {}
