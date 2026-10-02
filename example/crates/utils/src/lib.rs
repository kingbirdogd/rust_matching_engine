/// Utility functions library
use common;

pub fn format_result(label: &str, value: i32) -> String {
    common::log_info(&format!("Formatting: {}", label));
    format!("{}: {}", label, value)
}

pub fn print_separator() {
    println!("-------------------");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_result() {
        let result = format_result("answer", 42);
        assert!(result.contains("answer"));
        assert!(result.contains("42"));
    }
}
