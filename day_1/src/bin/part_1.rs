const INPUT: &str = include_str!("input.txt");

fn main() {
    let result = INPUT
        .lines()
        .map(|line| {
            let mut first_num = 0;
            let mut second_num = 0;
            for c in line.chars() {
                if c.is_numeric() {
                    first_num = c.to_digit(10).unwrap();
                    break;
                }
            }
            for c in line.chars().rev() {
                if c.is_numeric() {
                    second_num = c.to_digit(10).unwrap();
                    break;
                }
            }
            first_num * 10 + second_num
        })
        .sum::<u32>();

    println!("Result: {result}");
}
