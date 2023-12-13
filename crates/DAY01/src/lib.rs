pub mod error;

pub fn split_by_newline(s: &str) -> Vec<&str> {
    let split = s.trim().split('\n').collect::<Vec<&str>>();

    split
}

pub fn find_digit_byte_index(line: &str) -> Option<(usize, usize)> {
    let first_digi = line.find(|c: char| c.is_numeric());
    let last_digit = line.rfind(|c: char| c.is_numeric());

    match (first_digi, last_digit) {
        (Some(first_digi), Some(last_digit)) => Some((first_digi, last_digit)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static LINE_1: &str = "1abc2";
    static INPUT_1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

    #[test]
    fn split_into_lines() {
        println!("{:?}", INPUT_1);
        let result = split_by_newline(INPUT_1);
        assert_eq!(result[0], "1abc2");
    }

    #[test]
    fn process_line() {
        let result = find_digit_byte_index(LINE_1);
        println!("{:?}", result);
        assert_eq!(result, Some((0, 4)));
    }
}
