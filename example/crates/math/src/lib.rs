/// Math operations library
use common;

pub fn add(a: i32, b: i32) -> i32 {
    common::log_info(&format!("add({}, {})", a, b));
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    common::log_info(&format!("multiply({}, {})", a, b));
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
    }
}

