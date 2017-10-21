use super::ExitCode;

pub fn error(message: String) -> ExitCode {
    println!("Error: {}", message);

    ExitCode::Exit
}
