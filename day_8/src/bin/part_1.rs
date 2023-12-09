use std::collections::HashMap;

fn main() {
    let result = solution(include_str!("input.txt"));
    println!("Result: {result}");
}

fn solution(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();

    let sequence = lines[0]
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction char '{c}'"),
        })
        .collect::<Vec<_>>();

    let nodes = lines[2..]
        .iter()
        .map(|line| (&line[0..3], [&line[7..10], &line[12..15]]))
        .collect::<HashMap<&str, [&str; 2]>>();

    let mut node = "AAA";
    let mut n = 0;
    while node != "ZZZ" {
        let direction = sequence[n % sequence.len()];
        node = nodes[node][direction as usize];
        n += 1;
    }

    n
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let result = solution(include_str!("test_input1.txt"));
        assert_eq!(2, result);
        let result = solution(include_str!("test_input2.txt"));
        assert_eq!(6, result);
    }
}
