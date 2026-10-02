/// Common utilities shared across all crates
pub fn log_info(msg: &str) {
    println!("[INFO] {}", msg);
}

pub fn log_error(msg: &str) {
    eprintln!("[ERROR] {}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_info() {
        log_info("test");
    }
}
