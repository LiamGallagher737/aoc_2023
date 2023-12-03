use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

const RED_LIMIT: usize = 12;
const GREEN_LIMIT: usize = 13;
const BLUE_LIMIT: usize = 14;

fn main() {
    let result = INPUT
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .filter(|game| {
            for set in &game.sets {
                if set.red > RED_LIMIT || set.green > GREEN_LIMIT || set.blue > BLUE_LIMIT {
                    return false;
                }
            }
            true
        })
        .map(|game| game.id)
        .sum::<usize>();

    println!("Result: {result}");
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

#[derive(Debug, Default)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Game {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").collect::<Vec<_>>();
        let id = parts[0].split(" ").collect::<Vec<_>>()[1].parse::<usize>()?;
        let mut sets = Vec::new();
        for set_str in parts[1].split("; ") {
            let mut set = Set::default();
            for draw in set_str.split(", ") {
                let parts = draw.split(" ").collect::<Vec<_>>();
                let count = parts[0].parse::<usize>()?;
                match parts[1] {
                    "red" => set.red += count,
                    "green" => set.green += count,
                    "blue" => set.blue += count,
                    _ => {}
                };
            }
            sets.push(set);
        }
        Ok(Self {
            id,
            sets,
        })
    }
}
