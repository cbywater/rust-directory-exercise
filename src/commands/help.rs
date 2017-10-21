use super::ExitCode;

pub fn help() -> (ExitCode, String) {
    let message = format!("{}\n{}\n{}\n{}\n{}\n",
        "Usage:",
        "To add an employee use syntax 'add <FIRSTNAME> to <DEPARTMENT>'",
        "To list employees use 'list <DEPARTMENT>'",
        "To exit use 'exit'",
        "For this help message use 'help'"
    );

    (ExitCode::Continue, message)
}
