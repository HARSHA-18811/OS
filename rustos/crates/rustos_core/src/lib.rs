#![no_std]

extern crate alloc;

#[cfg(test)]
extern crate std;

use alloc::vec::Vec;

/// Evaluates a binary integer expression for `bc <n> <op> <n>`.
/// Supports +, -, *, /, % and negative numbers.
pub fn eval_bc(a: i64, op: &str, b: i64) -> Result<i64, &'static str> {
    match op {
        "+" => a.checked_add(b).ok_or("integer overflow"),
        "-" => a.checked_sub(b).ok_or("integer overflow"),
        "*" => a.checked_mul(b).ok_or("integer overflow"),
        "/" => {
            if b == 0 {
                Err("divide by zero")
            } else {
                a.checked_div(b).ok_or("integer overflow")
            }
        }
        "%" => {
            if b == 0 {
                Err("modulo by zero")
            } else {
                a.checked_rem(b).ok_or("integer overflow")
            }
        }
        _ => Err("unsupported operator (use +, -, *, /, %)"),
    }
}

/// Statistics for word count (`wc`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WcResult {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}

/// Counts lines, words, and bytes in a byte slice.
pub fn count_words(content: &[u8]) -> WcResult {
    let bytes = content.len();
    let mut lines = 0;
    let mut words = 0;
    let mut in_word = false;

    for &b in content {
        if b == b'\n' {
            lines += 1;
        }
        if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
            if in_word {
                words += 1;
                in_word = false;
            }
        } else {
            in_word = true;
        }
    }
    if in_word {
        words += 1;
    }

    WcResult {
        lines,
        words,
        bytes,
    }
}

/// Extracts columns from start to end (1-based inclusive) for a single line.
pub fn cut_line(line: &str, start: usize, end: usize) -> &str {
    if start == 0 || start > end || start > line.len() {
        return "";
    }
    let actual_start = start - 1;
    let actual_end = end.min(line.len());
    if actual_start >= actual_end {
        return "";
    }
    &line[actual_start..actual_end]
}

/// Returns the first `n` lines of text.
pub fn get_head<'a>(content: &'a str, n: usize) -> Vec<&'a str> {
    content.lines().take(n).collect()
}

/// Returns the last `n` lines of text.
pub fn get_tail<'a>(content: &'a str, n: usize) -> Vec<&'a str> {
    let all: Vec<&'a str> = content.lines().collect();
    let count = all.len();
    if count <= n {
        all
    } else {
        all[count - n..].to_vec()
    }
}

/// Filters lines containing the query word (`grab`).
pub fn grab_lines<'a>(content: &'a str, word: &str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(word))
        .collect()
}

/// Checks if a year is a leap year.
pub fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Returns number of days in a given month (1..=12).
pub fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Returns day of week for the 1st of the month (0 = Sunday, 1 = Monday, ..., 6 = Saturday).
pub fn first_day_of_month(year: u16, month: u8) -> u8 {
    static T: [u16; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 {
        y = y.saturating_sub(1);
    }
    let m_idx = (month.saturating_sub(1) as usize).min(11);
    ((y + y / 4 - y / 100 + y / 400 + T[m_idx] + 1) % 7) as u8
}

/// Month name string.
pub fn month_name(month: u8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_bc() {
        assert_eq!(eval_bc(10, "+", 20), Ok(30));
        assert_eq!(eval_bc(-15, "+", 5), Ok(-10));
        assert_eq!(eval_bc(100, "-", 45), Ok(55));
        assert_eq!(eval_bc(7, "*", 8), Ok(56));
        assert_eq!(eval_bc(-6, "*", -6), Ok(36));
        assert_eq!(eval_bc(100, "/", 10), Ok(10));
        assert_eq!(eval_bc(17, "%", 5), Ok(2));
        assert_eq!(eval_bc(10, "/", 0), Err("divide by zero"));
        assert_eq!(eval_bc(10, "%", 0), Err("modulo by zero"));
        assert_eq!(eval_bc(10, "^", 2), Err("unsupported operator (use +, -, *, /, %)"));
    }

    #[test]
    fn test_count_words() {
        let text = b"Hello world\nThis is RustOS\nTesting 1 2 3";
        let res = count_words(text);
        assert_eq!(res.lines, 2);
        assert_eq!(res.words, 9);
        assert_eq!(res.bytes, text.len());
    }

    #[test]
    fn test_cut_line() {
        assert_eq!(cut_line("abcdefgh", 1, 4), "abcd");
        assert_eq!(cut_line("abcdefgh", 3, 6), "cdef");
        assert_eq!(cut_line("abcdefgh", 5, 20), "efgh");
        assert_eq!(cut_line("abcdefgh", 10, 20), "");
    }

    #[test]
    fn test_head_and_tail() {
        let content = "line 1\nline 2\nline 3\nline 4\nline 5";
        let head = get_head(content, 2);
        assert_eq!(head, &["line 1", "line 2"]);

        let tail = get_tail(content, 2);
        assert_eq!(tail, &["line 4", "line 5"]);
    }

    #[test]
    fn test_grab_lines() {
        let content = "apple\nbanana\npineapple\norange\napple pie";
        let matches = grab_lines(content, "apple");
        assert_eq!(matches, &["apple", "pineapple", "apple pie"]);
    }

    #[test]
    fn test_calendar_helpers() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2026));
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2026, 2), 28);
        assert_eq!(days_in_month(2026, 11), 30);
    }
}
