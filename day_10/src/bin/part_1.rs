fn main() {
    let result = solution(include_str!("input.txt"));
    println!("Result: {result}");
}

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

    let mut position = start.expect("Unable to find start position");

    let mut direction = if matches!(maze.get(position.0).map(|row| row.get(position.1.wrapping_sub(1))), Some(Some(c)) if *c != '.' && *c != '|')
    {
        Direction::Left
    } else if matches!(maze.get(position.0).map(|row| row.get(position.1 + 1)), Some(Some(c)) if *c != '.' && *c != '|')
    {
        Direction::Right
    } else if matches!(maze.get(position.0 + 1).map(|row| row.get(position.1)), Some(Some(c)) if *c != '.' && *c != '-')
    {
        Direction::Up
    } else if matches!(maze.get(position.0.wrapping_sub(1)).map(|row| row.get(position.1)), Some(Some(c)) if *c != '.' && *c != '-')
    {
        Direction::Down
    } else {
        panic!("Unable to find pipe adjacent to start position: {position:?}");
    };
    position = new_position(position, direction);

    let mut pipe_count = 0;
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
                "Invalid direction state entered\nDirection: {direction:?}\nChar: {}",
                maze[position.0][position.1]
            ),
        };
        position = new_position(position, direction);
        pipe_count += 1;
    }

    (pipe_count + 1) / 2
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
        let result = solution(include_str!("test_input1.txt"));
        assert_eq!(4, result);
        let result = solution(include_str!("test_input2.txt"));
        assert_eq!(8, result);
    }
}
