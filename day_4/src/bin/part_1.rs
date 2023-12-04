use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let result = INPUT
        .lines()
        .map(Card::from_str)
        .map(Result::unwrap)
        .map(|card| {
            let mut winning_nums = 0;
            for num in card.have {
                if card.winning.contains(&num) {
                    winning_nums += 1;
                }
            }
            winning_nums
        })
        .filter(|winning_nums| 0 < *winning_nums)
        .map(|winning_nums| 2u32.pow(winning_nums - 1))
        .sum::<u32>();

    println!("Result: {result}");
}

struct Card {
    winning: Vec<u8>,
    have: Vec<u8>,
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.split(": ").collect::<Vec<_>>()[1];
        let parts = line.split(" | ").collect::<Vec<_>>();
        let winning = parts[0]
            .split_whitespace()
            .map(str::parse::<u8>)
            .map(Result::unwrap)
            .collect();
        let have = parts[1]
            .split_whitespace()
            .map(str::parse::<u8>)
            .map(Result::unwrap)
            .collect();
        Ok(Self { winning, have })
    }
}
