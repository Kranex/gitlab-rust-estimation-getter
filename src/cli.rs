use std::io::{stdin, stdout, Write};

pub fn prompt(msg: &str, default: Option<&str>) -> String {
    let mut input = String::new();

    match default {
        Some(value) => print!("{} (default: {}): ", msg, value),
        None        => print!("{}: ", msg),
    }

    let _ = stdout().flush();
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    if input.trim().is_empty() {
        return default.unwrap_or("").to_string();
    }

    return input.trim().to_string();
}

