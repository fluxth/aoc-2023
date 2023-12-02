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
    let iter = input.chars().filter(|char| char.is_digit(10));

    let tenths = iter.clone().next()?.to_digit(10)? * 10;
    let ones = iter.last()?.to_digit(10)?;

    Some(tenths + ones)
}
