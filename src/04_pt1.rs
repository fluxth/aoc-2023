use std::{
    collections::HashSet,
    io::{self, BufRead},
};

#[derive(Debug)]
#[allow(dead_code)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

fn main() {
    let stdin = io::stdin();
    let points: u64 = stdin
        .lock()
        .lines()
        .map(|line| {
            let input = line.expect("input line");
            let card = parse_card(&input);

            card.numbers
                .into_iter()
                .filter(|num| card.winning_numbers.contains(num))
                .enumerate()
                .map(|(i, _num)| 2_u64.pow(i.try_into().expect("i overflow")))
                .max()
                .unwrap_or(0)
        })
        .sum();

    println!("{points}");
}

fn parse_card(input: &str) -> Card {
    let (header, data) = input.split_once(": ").expect("card header");
    let (winning_numbers, card_numbers) = data.split_once(" | ").expect("card data");

    let id: u32 = header
        .split_whitespace()
        .last()
        .expect("card id")
        .parse()
        .expect("card id is number");

    Card {
        id,
        winning_numbers: winning_numbers
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("winning number is number"))
            .collect(),
        numbers: card_numbers
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("card number is number"))
            .collect(),
    }
}
