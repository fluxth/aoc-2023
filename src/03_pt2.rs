use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, BufRead},
};

type Matrix = Vec<Vec<char>>;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn main() {
    let stdin = io::stdin();
    let matrix = read_matrix(stdin.lock().lines());

    let mut gears: HashMap<Coordinate, HashMap<Coordinate, u32>> = HashMap::new();

    for (y, row) in matrix.iter().enumerate() {
        let mut x = 0;
        while x < row.len() {
            let char = row[x];

            x += match char {
                '0'..='9' => {
                    let number_string = &row[x..]
                        .iter()
                        .take_while(|char| char.is_digit(10))
                        .collect::<String>();

                    let number: u32 = number_string.parse().expect("able to parse number");
                    let number_len = number_string.len();

                    let number_coords = Coordinate { x, y };
                    for gear in find_gears(&matrix, x, y, number_len) {
                        match gears.entry(gear) {
                            Entry::Occupied(mut entry) => {
                                let number_map = entry.get_mut();
                                number_map.insert(number_coords, number);
                            }
                            Entry::Vacant(entry) => {
                                entry.insert(HashMap::from([(number_coords, number)]));
                            }
                        }
                    }

                    number_len
                }
                _ => 1,
            }
        }
    }

    let result: u64 = gears
        .values()
        .filter(|number_map| number_map.len() == 2)
        .map(|number_map| number_map.values().map(|num| *num as u64).product::<u64>())
        .sum();

    println!("{result}");
}

fn read_matrix(input: impl Iterator<Item = Result<String, io::Error>>) -> Matrix {
    input
        .map(|line| line.expect("able to read line").chars().collect())
        .collect()
}

fn find_gears(matrix: &Matrix, x: usize, y: usize, len: usize) -> Vec<Coordinate> {
    let x: i64 = x.try_into().expect("x overflow");
    let y: i64 = y.try_into().expect("y overflow");
    let len: i64 = len.try_into().expect("len overflow");

    let mut to_check = vec![
        (x - 1, y - 1),   // n-w
        (x - 1, y),       // w
        (x - 1, y + 1),   // s-w
        (x + len, y - 1), // n-e
        (x + len, y),     // e
        (x + len, y + 1), // s-e
    ];

    for index in 0..len {
        let i: i64 = index.try_into().expect("i overflow");

        to_check.push((x + i, y - 1)); // top
        to_check.push((x + i, y + 1)); // bottom
    }

    to_check
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .filter_map(|(x, y)| -> Option<Coordinate> {
            let x: usize = x.try_into().expect("x overflow");
            let y: usize = y.try_into().expect("y overflow");

            if *matrix.get(y)?.get(x)? == '*' {
                Some(Coordinate { x, y })
            } else {
                None
            }
        })
        .collect()
}
