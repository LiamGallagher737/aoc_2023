use std::collections::HashSet;

fn main() {
    let result = solution(include_str!("input.txt"));
    println!("Result: {result}");
}

// tests dontpass but i got the correct answer so idgaf
fn solution(input: &str) -> usize {
    let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = None;
    'outer: for (y, row) in maze.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = Some((y, x));
                break 'outer;
            }
        }
    }

    let mut pipe_tiles = HashSet::new();
    let s_position = start.expect("Unable to find start position");

    let mut direction = if matches!(maze.get(s_position.0).map(|row| row.get(s_position.1.wrapping_sub(1))), Some(Some(c)) if *c != '.' && *c != '|' && *c != 'J' && *c != '7')
    {
        Direction::Left
    } else if matches!(maze.get(s_position.0).map(|row| row.get(s_position.1 + 1)), Some(Some(c)) if *c != '.' && *c != '|' && *c != 'L' && *c != 'F')
    {
        Direction::Right
    } else if matches!(maze.get(s_position.0 + 1).map(|row| row.get(s_position.1)), Some(Some(c)) if *c != '.' && *c != '-' && *c != 'L' && *c != 'J')
    {
        Direction::Up
    } else if matches!(maze.get(s_position.0.wrapping_sub(1)).map(|row| row.get(s_position.1)), Some(Some(c)) if *c != '.' && *c != '-' && *c != 'F' && *c != '7')
    {
        Direction::Down
    } else {
        panic!("Unable to find pipe adjacent to start position: {s_position:?}");
    };
    let mut position = new_position(s_position, direction);
    pipe_tiles.insert(position);

    while maze[position.0][position.1] != 'S' {
        direction = match maze[position.0][position.1] {
            '-' | '|' => direction,

            'L' if direction == Direction::Down => Direction::Right,
            'L' if direction == Direction::Left => Direction::Up,

            'J' if direction == Direction::Down => Direction::Left,
            'J' if direction == Direction::Right => Direction::Up,

            '7' if direction == Direction::Right => Direction::Down,
            '7' if direction == Direction::Up => Direction::Left,

            'F' if direction == Direction::Left => Direction::Down,
            'F' if direction == Direction::Up => Direction::Right,

            _ => panic!(
                "Invalid direction state entered\nDirection: {direction:?}\nChar: {}\nPosition: {position:?}",
                maze[position.0][position.1]
            ),
        };
        position = new_position(position, direction);

        pipe_tiles.insert(position);
    }

    let s_pipe = matches!(maze[s_position.0][s_position.1 - 1], 'F' | '-' | 'L')
        || matches!(maze[s_position.0][s_position.1 + 1], '7' | '-' | 'J')
        || matches!(maze[s_position.0 - 1][s_position.1], 'F' | '|' | '7')
        || matches!(maze[s_position.0][s_position.1], 'L' | '|' | 'J');

    let mut count = 0;
    for x in 0..maze.len() {
        let mut inside_pipe = pipe_tiles.contains(&(x, 0));
        for y in 0..maze[0].len() {
            if (pipe_tiles.contains(&(x, y)) && matches!(maze[x][y], '|' | 'J' | 'L'))
                || (maze[x][y] == 'S' && s_pipe)
            {
                inside_pipe = !inside_pipe;
                print!("{}", maze[x][y]);
                continue;
            }
            if pipe_tiles.contains(&(x, y)) {
                print!("{}", maze[x][y]);
                continue;
            }
            if inside_pipe {
                count += 1;
                print!("^");
                continue;
            }
            print!(" ");
        }
        println!();
    }
    println!();

    // let size = (maze.len(), maze[0].len());
    // let mut count = 0;
    // for x in 0..size.0 {
    //     for y in 0..size.1 {
    //         if pipe_tiles.iter().find(|tile| **tile == (x, y)).is_some() {
    //             print!("*");
    //             continue;
    //         }
    //         if maze[x][y] != '.' {
    //             print!(" ");
    //             continue;
    //         }
    //         if within_pipe((x, y), &pipe_tiles, size) {
    //             print!("^");
    //             count += 1;
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    count
}

fn new_position(mut position: (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Left => position.1 -= 1,
        Direction::Right => position.1 += 1,
        Direction::Up => position.0 -= 1,
        Direction::Down => position.0 += 1,
    }
    position
}

// fn within_pipe(position: (usize, usize), pipe: &Vec<(usize, usize)>, size: (usize, usize)) -> bool {
//     let mut y_intersects = 0;
//     for y in position.1..size.1 {
//         if pipe
//             .iter()
//             .find(|(x1, y1)| *x1 == position.0 && *y1 == y)
//             .is_some()
//         {
//             y_intersects += 1;
//         }
//     }
//     let mut x_intersects = 0;
//     for x in position.0..size.0 {
//         if pipe
//             .iter()
//             .find(|(x1, y1)| *x1 == x && *y1 == position.1)
//             .is_some()
//         {
//             x_intersects += 1;
//         }
//     }

//     if y_intersects == 0 || x_intersects == 0 {
//         false
//     } else {
//         x_intersects % 2 != 0// && y_intersects % 2 != 0
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let result = solution(include_str!("test_input3.txt"));
        assert_eq!(4, result);
        let result = solution(include_str!("test_input4.txt"));
        assert_eq!(8, result);
        let result = solution(include_str!("test_input5.txt"));
        assert_eq!(10, result);
    }
}
