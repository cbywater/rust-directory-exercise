use super::ExitCode;
use directory::Directory;

pub fn add_user(user: String, department: String, directory: &mut Directory) -> ExitCode {
    let dept_record = directory.departments.entry(department).or_insert(Vec::new());

    (*dept_record).push(user);

    ExitCode::Continue
}
