use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    io::{self, BufRead},
};

#[derive(Debug)]
#[allow(dead_code)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
    win_count: u32,
}

fn main() {
    let stdin = io::stdin();
    let cards: BTreeMap<u32, Card> = stdin
        .lock()
        .lines()
        .map(|line| {
            let input = line.expect("input line");
            let card = parse_card(&input);

            (card.id, card)
        })
        .collect();

    let mut card_queue: VecDeque<&Card> = cards.values().collect();
    let mut card_count = 0;

    while let Some(card) = card_queue.pop_front() {
        card_count += 1;

        for card_id in card.id + 1..(card.id + 1 + card.win_count) {
            if let Some(copied_card) = cards.get(&card_id) {
                card_queue.push_back(copied_card);
            } else {
                unreachable!();
            }
        }
    }

    println!("{card_count}");
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

    let winning_numbers: HashSet<u32> = winning_numbers
        .split_whitespace()
        .map(|num| num.parse::<u32>().expect("winning number is number"))
        .collect();

    let numbers: Vec<u32> = card_numbers
        .split_whitespace()
        .map(|num| num.parse::<u32>().expect("card number is number"))
        .collect();

    let win_count: u32 = numbers
        .iter()
        .filter(|num| winning_numbers.contains(num))
        .count()
        .try_into()
        .expect("match count overflow");

    Card {
        id,
        winning_numbers,
        numbers,
        win_count,
    }
}
