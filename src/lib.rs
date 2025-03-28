use chrono::Local;

pub mod parser;
pub mod stop_watch;

#[allow(dead_code)]
fn get_now_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

pub fn help() {
    println!("\nthis is help page!\n");

    let commands = vec![
        "help: Help Page",
        "stopwatch: Use StopWatch",
        "timer: Use Timer",
        "quit: Quit rust-timer",
    ];

    println!("{}", commands.join("\n"));
}
