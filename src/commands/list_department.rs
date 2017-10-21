use super::ExitCode;
use directory::Directory;

pub fn list_department(department: String, directory: &mut Directory) -> (ExitCode, String) {
    match directory.departments.get_mut(&department) {
        Some(names) => {
            names.sort_unstable();

            let mut message = format!("The {} department includes:\n", department);

            for name in names {
                message = format!("{}    {}\n", message, name);
            }

            (ExitCode::Continue, message)
        },

        None => {
            let message = format!("There are no people in the {} department\n", department);

            (ExitCode::Continue, message)
        },
    }
}
