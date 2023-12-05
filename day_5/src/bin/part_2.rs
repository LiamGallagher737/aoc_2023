use std::{num::ParseIntError, str::FromStr};

fn main() {
    let result = solution(include_str!("input.txt"));
    println!("Result: {result}");
}

fn solution(input: &str) -> usize {
    // i realize this isn't the correct way to do it but i cbf fixing it
    // run it in release mode
    let lines = input.lines().collect::<Vec<_>>();
    let seeds = lines[0][7..]
        .split_whitespace()
        .map(|num| num.parse::<usize>())
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|pair| {
            let mut list = Vec::with_capacity(pair[1]);
            for n in 0..pair[1] {
                list.push(pair[0] + n)
            }
            list.into_iter()
        })
        .flatten()
        .collect::<Vec<_>>();

    let mapss = lines[2..]
        .split(|line| line.is_empty())
        .map(|section| {
            section[1..]
                .iter()
                .map(|line| Map::from_str(line))
                .map(Result::unwrap)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut counters = seeds;
    for maps in mapss {
        for count in &mut counters {
            for map in &maps {
                if let Some(mapped_num) = map.map(*count) {
                    *count = mapped_num;
                    break;
                }
            }
        }
    }

    *counters.iter().min().unwrap()
}

struct Map {
    source_start: usize,
    destination_start: usize,
    range: usize,
}

impl Map {
    fn map(&self, source: usize) -> Option<usize> {
        if self.source_start <= source && source < self.source_start + self.range {
            let dif = self.destination_start as isize - self.source_start as isize;
            Some((source as isize + dif) as usize)
        } else {
            None
        }
    }
}

impl FromStr for Map {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        Ok(Self {
            source_start: parts[1].parse()?,
            destination_start: parts[0].parse()?,
            range: parts[2].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let result = solution(include_str!("test_input.txt"));
        assert_eq!(46, result);
    }

    #[test]
    fn test_map() {
        let map = Map::from_str("50 98 2").expect("Failed to parse map");
        assert_eq!(None, map.map(13));
        assert_eq!(Some(50), map.map(98));
        assert_eq!(Some(51), map.map(99));
        assert_eq!(None, map.map(100));
    }
}
