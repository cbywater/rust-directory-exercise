use super::ExitCode;
use directory::Directory;

pub fn list_department(department: String, directory: &mut Directory) -> ExitCode {
    match directory.departments.get_mut(&department) {
        Some(names) => {
            names.sort_unstable();

            println!("The {} department includes:", department);

            for name in names {
                println!("    {}", name);
            }

            ExitCode::Continue
        },

        None => {
            println!("There are no people in the {} department", department);

            ExitCode::Continue
        },
    }
}
