const INPUT: &str = include_str!("input.txt");

const NUMBER_WORDS: &[(&str, u32)] = &[
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let result = INPUT
        .lines()
        .map(|line| {
            let mut first_num = 0;
            let mut second_num = 0;
            let mut seen_chars = String::new();
            'line: for c in line.chars() {
                if c.is_numeric() {
                    first_num = c.to_digit(10).unwrap();
                    break;
                }
                seen_chars.push(c);
                for (word, num) in NUMBER_WORDS {
                    if seen_chars.contains(word) {
                        first_num = *num;
                        break 'line;
                    }
                }
            }
            seen_chars = String::new();
            'line: for c in line.chars().rev() {
                if c.is_numeric() {
                    second_num = c.to_digit(10).unwrap();
                    break;
                }
                seen_chars.push(c);
                for (word, num) in NUMBER_WORDS {
                    let rev = word.chars().rev().collect::<String>();
                    if seen_chars.contains(&rev) {
                        second_num = *num;
                        break 'line;
                    }
                }
            }
            first_num * 10 + second_num
        })
        .sum::<u32>();

    println!("Result: {result}");
}
