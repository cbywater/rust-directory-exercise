use std::io;
use std::collections::HashMap;

mod parser;
mod commands;
mod directory;

use parser::parse_input;
use commands::ExitCode;
use directory::Directory;

fn main() {
    let mut directory = Directory {
        departments: HashMap::new(),
    };

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let command = parse_input(input);
        let (exit_code, message) = command.execute(&mut directory);

        print!("{}", message);

        if let ExitCode::Exit = exit_code {
            break;
        }
    }
}
