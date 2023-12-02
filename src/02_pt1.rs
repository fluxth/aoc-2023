use std::{
    collections::HashMap,
    io::{self, BufRead},
};

const BAG_CONDITION: [(&'static str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn main() {
    let stdin = io::stdin();
    let id_sum: u32 = stdin
        .lock()
        .lines()
        .map(|line| {
            let input = line.expect("input line");
            let game = parse_game(&input);

            for (color, count) in BAG_CONDITION {
                if *game.colors_max.get(color).unwrap_or(&0) > count {
                    return 0;
                }
            }

            game.id
        })
        .sum();

    println!("{id_sum}");
}

#[derive(Debug)]
struct Game<'a> {
    id: u32,
    colors_max: HashMap<&'a str, u32>,
}

fn parse_game(game_line: &str) -> Game {
    let mut colors_max = HashMap::new();

    let (game_id, game_line) = game_line.split_once(':').expect("game id header");
    let (_, game_id) = game_id.split_once(' ').expect("game id number");

    for draw in game_line.split(';') {
        let draw = draw.trim();
        for item in draw.split(',') {
            let item = item.trim();

            let (count, color) = item.split_once(' ').expect("item count and color");
            let count: u32 = count.parse().expect("count to be digit");

            let stored_count = *colors_max.entry(color).or_insert(0);
            if count > stored_count {
                colors_max.insert(color, count);
            }
        }
    }

    Game {
        id: game_id.parse().expect("game id to be digit"),
        colors_max,
    }
}
