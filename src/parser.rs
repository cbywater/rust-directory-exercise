use super::commands::Command;

pub fn parse_input(input: String) -> Command {
    let mut words = Vec::new();

    for word in input.split_whitespace() {
        words.push(word);
    }

    match words.get(0) {
        Some(&"add") => parse_add(words),
        Some(&"list") => parse_list(words),
        Some(&"help") => Command::Help,
        Some(&"exit") => Command::Exit,
        None => Command::Help,
        _ => Command::Error(String::from("Invalid input")),
    }
}

fn parse_add(words: Vec<&str>) -> Command {
    if words.len() != 4 {
        Command::Error(String::from("Invalid input length"))
    } else if words.get(2) != Some(&"to") {
        Command::Error(String::from("Invalid command"))
    } else {
        let user = match words.get(1) {
            Some(u) => String::from(*u),
            None => String::new(),
        };

        let dept = match words.get(3) {
            Some(d) => String::from(*d),
            None => String::new(),
        };

        Command::AddUser(user, dept)
    }
}

fn parse_list(words: Vec<&str>) -> Command {
    if words.len() != 2 {
        Command::Error(String::from("Invalid input length"))
    } else {
        let dept = match words.get(1) {
            Some(d) => String::from(*d),
            None => String::new(),
        };

        Command::ListDepartment(dept)
    }
}
