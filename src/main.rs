use parser::{parse, run_command};
use std::io;
use std::io::Write;

mod parser;

fn main() {
    loop {
        print!("\nrust-timer: ");
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
