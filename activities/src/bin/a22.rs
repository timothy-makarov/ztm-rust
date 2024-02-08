// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test_suite {
    use crate::*;

    #[test]
    fn test_clamp_1_0_2() {
        assert_eq!(clamp(1, 0, 2), 1);
    }

    #[test]
    fn test_clamp_m1_0_2() {
        assert_eq!(clamp(-1, 0, 2), 0);
    }

    #[test]
    fn test_clamp_300_0_2() {
        assert_eq!(clamp(300, 0, 2), 2);
    }

    #[test]
    fn test_clamp_0_0_2() {
        assert_eq!(clamp(0, 0, 2), 0);
    }

    #[test]
    fn test_clamp_2_0_2() {
        assert_eq!(clamp(2, 0, 2), 2);
    }

    #[test]
    fn test_clamp_m2_m4_m1() {
        assert_eq!(clamp(-2, -4, -1), -2);
    }

    #[test]
    fn test_clamp_m5_m4_m1() {
        assert_eq!(clamp(-5, -4, -1), -4);
    }

    #[test]
    fn test_div_4_2() {
        assert_eq!(div(4, 2), Some(2));
    }

    #[test]
    fn test_div_0_2() {
        assert_eq!(div(0, 2), Some(0));
    }

    #[test]
    fn test_div_4_0() {
        assert_eq!(div(4, 0), None);
    }

    #[test]
    fn test_concat_empty() {
        assert_eq!(concat("", ""), "");
    }

    #[test]
    fn test_concat_foo_bar() {
        assert_eq!(concat("foo", "bar"), "foobar");
    }

    #[test]
    fn test_concat_123_qwerty() {
        assert_eq!(concat("123", "qwerty"), "123qwerty");
    }

    #[test]
    fn test_concat_hello_empty() {
        assert_eq!(concat("hello", ""), "hello");
    }

    #[test]
    fn test_concat_empty_hello() {
        assert_eq!(concat("", "hello"), "hello");
    }
}
