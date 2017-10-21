use super::directory::Directory;

mod error;
mod help;
mod exit;
mod list_department;
mod add_user;

use self::error::error;
use self::help::help;
use self::exit::exit;
use self::list_department::list_department;
use self::add_user::add_user;

pub enum Command {
    Exit,
    AddUser(String, String),
    ListDepartment(String),
    Help,
    Error(String),
}

impl Command {
    pub fn execute(&self, mut directory: &mut Directory) -> ExitCode {
        match self {
            &Command::AddUser(ref user, ref dept) => add_user(user.clone(), dept.clone(), &mut directory),
            &Command::ListDepartment(ref dept) => list_department(dept.clone(), &mut directory),
            &Command::Exit => exit(),
            &Command::Help => help(),
            &Command::Error(ref message) => error(message.clone()),
        }
    }
}

pub enum ExitCode {
    Continue,
    Exit,
}
