fn main() {
    let result = solution(include_str!("input.txt"));
    println!("Result: {result}");
}

fn solution(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let time = lines[0][11..].replace(" ", "").parse::<usize>().unwrap();
    let best_distance = lines[1][11..].replace(" ", "").parse::<usize>().unwrap();

    let mut start = 0;
    for time_held in 1..time {
        let distance = time_held * (time - time_held);
        if distance > best_distance {
            start = time_held;
            break;
        }
    }

    let mut end = 0;
    for time_held in (1..time).rev() {
        let distance = time_held * (time - time_held);
        if distance > best_distance {
            end = time_held;
            break;
        }
    }

    end - start + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let result = solution(include_str!("test_input.txt"));
        assert_eq!(71503, result);
    }
}
