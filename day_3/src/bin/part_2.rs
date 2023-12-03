use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let schematic = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let get_entire_number = |x: usize, y: usize| {
        let line = &schematic[x];
        let mut start = y;
        while start > 0 && line[start - 1].is_numeric() {
            start -= 1;
        }
        let mut end = y;
        while end < schematic[x].len() - 1 && line[end+1].is_numeric() {
            end += 1;
        }
        line[start..=end]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    };

    let mut total = 0;
    for x in 0..schematic.len() {
        for y in 0..schematic[0].len() {
            if schematic[x][y] != '*' {
                continue;
            }

            let mut nums = HashSet::new();

            if 0 < y {
                if schematic[x][y - 1].is_numeric() {
                    nums.insert(get_entire_number(x, y - 1));
                }
                if 0 < x && schematic[x - 1][y - 1].is_numeric() {
                    nums.insert(get_entire_number(x - 1, y - 1));
                }
                if x < schematic.len() - 1 && schematic[x + 1][y - 1].is_numeric() {
                    nums.insert(get_entire_number(x + 1, y - 1));
                }
            }
            if y < schematic[x].len() - 1 {
                if schematic[x][y + 1].is_numeric() {
                    nums.insert(get_entire_number(x, y + 1));
                }
                if 0 < x && schematic[x - 1][y + 1].is_numeric() {
                    nums.insert(get_entire_number(x - 1, y + 1));
                }
                if x < schematic.len() - 1 && schematic[x + 1][y + 1].is_numeric() {
                    nums.insert(get_entire_number(x + 1, y + 1));
                }
            }
            if 0 < x && schematic[x - 1][y].is_numeric() {
                nums.insert(get_entire_number(x - 1, y));
            }
            if 0 < x && schematic[x + 1][y].is_numeric() {
                nums.insert(get_entire_number(x + 1, y));
            }

            if nums.len() == 2 {
                total += nums.iter().product::<usize>();
            }
        }
    }

    println!("Result: {total}");
}
