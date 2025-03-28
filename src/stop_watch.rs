use std::{
    io::{self, stdout, Write},
    sync::Mutex,
    thread::sleep,
    time::Duration,
};

use chrono::{DateTime, Local};
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use figlet_rs::FIGfont;
use lazy_static::lazy_static;

lazy_static! {
    static ref Time: Mutex<Vec<DateTime<Local>>> = Mutex::new(Vec::new());
}

enum Command {
    Enter,
    Help,
    Start,
    Stop,
    End,
    Unknown(String),
}

fn parse(arg: &str) -> Command {
    match arg {
        "\n" => Command::Enter,
        "help" => Command::Help,
        "start" => Command::Start,
        "stop" => Command::Stop,
        "end" => Command::End,
        _ => Command::Unknown(arg.to_string()),
    }
}

fn run_command(command: Command) {
    match command {
        Command::Enter => {}
        Command::Help => crate::help(),
        Command::Start => start(),
        Command::Stop => stop(),
        Command::End => end(),
        Command::Unknown(cmd) => println!("command not found: {}", cmd),
    }
}

fn start() {
    let start = Local::now();
    let mut time = Time.lock().unwrap();
    time.push(start);
    let start = time[0];

    let mut stdout = stdout();
    let standard_font = FIGfont::standard().unwrap();

    loop {
        let now = Local::now();
        let diff = format!("{}", now - start);
        let v: Vec<&str> = diff.split("").collect();
        let diff = v[3..v.len() - 3].join("");
        let funcition_figure = standard_font.convert("stopwatch").unwrap();
        let time_figure = standard_font.convert(&diff).unwrap();

        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All)).unwrap();
        println!("{}", funcition_figure);
        println!("{}", time_figure);
        stdout.flush().expect("Failed to flush");
        sleep(Duration::from_millis(100));
    }
}

fn stop() {
    unimplemented!();
}

fn end() {
    unimplemented!();
}

pub fn stop_watch() {
    println!("Please input start command! (rust-timer: start)");

    loop {
        print!("stop-watch: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => {
                let arg = buf.trim();
                let command = parse(arg);
                run_command(command);
            }
            Err(e) => eprintln!("Error reading input: {}", e),
        }
    }
}
