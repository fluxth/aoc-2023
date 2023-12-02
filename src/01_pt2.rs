use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let result: u32 = stdin
        .lock()
        .lines()
        .map(|line| {
            let input = line.expect("input line");
            trebuchet(&input).expect("first and last digit")
        })
        .sum();

    println!("{}", result);
}

fn trebuchet(input: &str) -> Option<u32> {
    let mut iter = DigitResolver::new(input).peekable();

    let tenths = iter.peek()? * 10;
    let ones = iter.last()?;

    Some(tenths + ones)
}

#[derive(Clone, Copy)]
struct DigitResolver<'a> {
    buf: &'a str,
}

macro_rules! check_and_return_worded_digit {
    ($buf: ident, $digit: literal, $word: literal) => {
        if $buf.starts_with($word) {
            return Some($digit);
        }
    };
}

impl Iterator for DigitResolver<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(char) = self.buf.chars().next() {
            let buf = self.buf;
            self.buf = &self.buf[char.len_utf8()..];

            if let Some(digit) = char.to_digit(10) {
                return Some(digit);
            }

            check_and_return_worded_digit!(buf, 1, "one");
            check_and_return_worded_digit!(buf, 2, "two");
            check_and_return_worded_digit!(buf, 3, "three");
            check_and_return_worded_digit!(buf, 4, "four");
            check_and_return_worded_digit!(buf, 5, "five");
            check_and_return_worded_digit!(buf, 6, "six");
            check_and_return_worded_digit!(buf, 7, "seven");
            check_and_return_worded_digit!(buf, 8, "eight");
            check_and_return_worded_digit!(buf, 9, "nine");
        }

        assert!(self.buf.len() == 0);

        None
    }
}

impl<'a> DigitResolver<'a> {
    fn new(input: &'a str) -> Self {
        Self { buf: input }
    }
}
