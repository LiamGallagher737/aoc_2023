const INPUT: &str = include_str!("input.txt");

fn main() {
    let schematic = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut total = 0;
    for x in 0..schematic.len() {
        let mut seen_numbers_count = 0;
        for y in 0..schematic[0].len() {
            if schematic[x][y].is_numeric() {
                seen_numbers_count += 1;
            }
            if seen_numbers_count > 0
                && (y == schematic[x].len() - 1 || !schematic[x][y + 1].is_numeric())
            {
                let mut chars = Vec::new();

                if seen_numbers_count <= y {
                    chars.push(schematic[x].get(y - seen_numbers_count));
                    if x > 0 {
                        chars.push(schematic[x - 1].get(y - seen_numbers_count));
                    }
                    if x < schematic.len() - 1 {
                        chars.push(schematic[x + 1].get(y - seen_numbers_count));
                    }
                }
                chars.push(schematic[x].get(y + 1));
                if x > 0 {
                    chars.push(schematic[x - 1].get(y + 1));
                }
                if x < schematic.len() - 1 {
                    chars.push(schematic[x + 1].get(y + 1));
                }

                for h in y + 1 - seen_numbers_count..=y {
                    if x > 0 {
                        chars.push(schematic[x - 1].get(h));
                    }
                    if x < schematic.len() - 1 {
                        chars.push(schematic[x + 1].get(h));
                    }
                }

                let is_part = chars
                    .iter()
                    .filter_map(|c| *c)
                    .any(|c| c.is_ascii_punctuation() && *c != '.');

                if is_part {
                    let num_chars = &schematic[x][y + 1 - seen_numbers_count..y+1];
                    let num = num_chars
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    total += num;
                }

                seen_numbers_count = 0;
            }
        }
    }

    println!("Result: {total}");
}
