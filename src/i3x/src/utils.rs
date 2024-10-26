pub fn format_output(msg: &str) -> String {
    format!("--- {} ---", msg)
}

pub fn log_action(action: &str) {
    println!("Action: {}", action);
}
