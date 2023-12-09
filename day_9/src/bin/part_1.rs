fn main() {
    let result = solution(include_str!("input.txt"));
    println!("Result: {result}");
}

fn solution(input: &str) -> isize {
    let mut histories = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<isize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    histories
        .drain(..)
        .map(|history| calc_steps(vec![history]))
        .map(|steps| steps.iter().map(|v| v.last()).map(Option::unwrap).sum::<isize>())
        .sum()
}

fn calc_steps(mut sequence: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let last = sequence.last().unwrap();
    let mut steps = Vec::with_capacity(last.len() - 1);
    for pair in last.windows(2) {
        steps.push(pair[1] - pair[0]);
    }
    if steps.iter().all(|s| *s == 0) {
        sequence
    } else {
        sequence.push(steps);
        calc_steps(sequence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let result = solution(include_str!("test_input.txt"));
        assert_eq!(114, result);
    }
}
