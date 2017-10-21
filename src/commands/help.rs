use super::ExitCode;

pub fn help() -> ExitCode {
    println!("Usage:");
    println!("To add an employee use syntax 'add <FIRSTNAME> to <DEPARTMENT>'");
    println!("To list employees use 'list <DEPARTMENT>'");
    println!("To exit use 'exit'");
    println!("For this help message use 'help'");

    ExitCode::Continue
}
