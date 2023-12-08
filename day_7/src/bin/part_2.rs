use std::{cmp::Ordering, collections::HashMap, str::FromStr};

fn main() {
    let result = solution(include_str!("input.txt"));
    println!("Result: {result}");
}

fn solution(input: &str) -> usize {
    let mut cards = input
        .lines()
        .map(Hand::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    cards.sort();
    cards
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum()
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts_map = HashMap::new();
        for c in &self.cards {
            let count = counts_map.entry(c).or_insert(0);
            *count += 1;
        }
        let mut counts = Vec::new();
        for (_, count) in counts_map {
            counts.push(count);
        }
        counts.sort_by(|a, b| b.cmp(a));
        let j_count = self.cards.iter().filter(|c| **c == Card::J).count();
        if j_count > 0 && counts.len() > 1 {
            let j_index = counts.iter().position(|c| *c == j_count).unwrap();
            counts.remove(j_index);
            counts[0] += j_count;
        }

        match &counts[0..] {
            &[5] => HandType::FiveOfAKind,
            &[4, 1] => HandType::FourOfAKind,
            &[3, 2] => HandType::FullHouse,
            &[3, 1, 1] => HandType::ThreeOfAKind,
            &[2, 2, 1] => HandType::TwoPair,
            &[2, 1, 1, 1] => HandType::OnePair,
            &[1, 1, 1, 1, 1] => HandType::HeighFive,
            _ => panic!("Invalid hand type: {counts:?}"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => Some(self.cards.cmp(&other.cards)),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let cards = parts[0]
            .chars()
            .map(Card::try_from)
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        Ok(Self {
            cards: cards.try_into().unwrap(),
            bid: parts[1].parse().unwrap(),
        })
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HeighFive = 0,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
enum Card {
    A = 12,
    K = 11,
    Q = 10,
    T = 9,
    N9 = 8,
    N8 = 7,
    N7 = 6,
    N6 = 5,
    N5 = 4,
    N4 = 3,
    N3 = 2,
    N2 = 1,
    J = 0,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::A),
            'K' => Ok(Self::K),
            'Q' => Ok(Self::Q),
            'J' => Ok(Self::J),
            'T' => Ok(Self::T),
            '9' => Ok(Self::N9),
            '8' => Ok(Self::N8),
            '7' => Ok(Self::N7),
            '6' => Ok(Self::N6),
            '5' => Ok(Self::N5),
            '4' => Ok(Self::N4),
            '3' => Ok(Self::N3),
            '2' => Ok(Self::N2),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let result = solution(include_str!("test_input.txt"));
        assert_eq!(5905, result);
    }
}
