use super::ExitCode;

pub fn error(error_message: String) -> (ExitCode, String) {
    let message = format!("Error: {}\n", error_message);

    (ExitCode::Exit, message)
}
