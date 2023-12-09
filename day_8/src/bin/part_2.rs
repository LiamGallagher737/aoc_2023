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

    let mut current_nodes = nodes
        .keys()
        .filter(|node| node.ends_with("A"))
        .collect::<Vec<_>>();
    let mut counts = vec![usize::MAX; current_nodes.len()];
    let mut n = 0;
    while counts.iter().any(|count| *count == usize::MAX) {
        let direction = sequence[n % sequence.len()];
        for i in 0..current_nodes.len() {
            current_nodes[i] = &nodes[*current_nodes[i]][direction as usize];
            if current_nodes[i].ends_with("Z") {
                counts[i] = n + 1;
            }
        }
        n += 1;
    }

    list_lcm(&counts)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn list_lcm(numbers: &Vec<usize>) -> usize {
    let mut result = numbers[0];
    for &num in &numbers[1..] {
        result = lcm(result, num);
    }
    result
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
        let result = solution(include_str!("test_input3.txt"));
        assert_eq!(6, result);
    }
}
