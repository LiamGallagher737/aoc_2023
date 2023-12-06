fn main() {
    let result = solution(include_str!("input.txt"));
    println!("Result: {result}");
}

fn solution(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let times = lines[0][11..]
        .split_whitespace()
        .map(str::parse::<usize>)
        .map(Result::unwrap);
    let distances = lines[1][11..]
        .split_whitespace()
        .map(str::parse::<usize>)
        .map(Result::unwrap);

    times
        .zip(distances)
        .map(|(time, best_distance)| {
            (1..time)
                .map(|time_held| time_held * (time - time_held))
                .filter(|distance| *distance > best_distance)
                .count()
        })
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let result = solution(include_str!("test_input.txt"));
        assert_eq!(288, result);
    }
}
