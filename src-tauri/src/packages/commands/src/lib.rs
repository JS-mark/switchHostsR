use tauri::command;

#[command]
pub fn hi(name: &str) -> String {
    return format!("Hello, {}! You've been greeted from Rust!", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
