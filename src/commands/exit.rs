use super::ExitCode;

pub fn exit() -> (ExitCode, String) {
    (ExitCode::Exit, String::from(""))
}
