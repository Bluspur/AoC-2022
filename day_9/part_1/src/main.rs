use std::{fs, collections::BTreeSet};

const INPUT_PATH: &str = "..\\assets\\input.txt";
const TEST_PATH: &str = "..\\assets\\test.txt";

struct Segment {
    position: (i32, i32),
}

impl Segment {
    fn new(x: i32, y: i32) -> Self {
        Segment { position: (x,y) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
    Up,
    Down,
    Left,
    Right,
}

impl Command {
    fn from_char(char: char) -> Result<Self, String> {
        match char {
            'U' => Ok(Command::Up),
            'D' => Ok(Command::Down),
            'L' => Ok(Command::Left),
            'R' => Ok(Command::Right),
            _ => Err("character is not one of U D L or R".to_string()),
        }
    }
    fn get_value(&self) -> (i32, i32) {
        match self {
            Command::Up => (0,1),
            Command::Down => (0,-1),
            Command::Left => (-1,0),
            Command::Right => (1,0),
        }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).expect("Failed to parse file to string");

    let output = solve(&input);

    println!("Output: {}", output);
}

fn solve(input: &str) -> u32 {
    let commands = parse_string_to_commands(input);
    return execute_commands(commands);
}

fn parse_string_to_commands(input: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    for line in input.lines() {
        let (raw_command, raw_repetition_count) = line.split_at(2);

        let command_identifier = raw_command.chars().next().unwrap();
        let command = Command::from_char(command_identifier).unwrap();

        let repetition_count: u32 = raw_repetition_count.parse().expect("repetition count should consist of valid numeric characters");

        for _ in 0..repetition_count {
            commands.push(command.clone());
        }
    }
    commands
}

fn execute_commands(commands: Vec<Command>) -> u32 {
    let mut rope = Vec::new();

    for _ in 0..2 {
        rope.push(Segment::new(0,0));
    }

    let mut visited_tail_positions = BTreeSet::new();

    for command in commands {
        let mut head = &mut rope[0];
        head.position.0 += command.get_value().0;
        head.position.1 += command.get_value().1;

        for i in 1..rope.len() {
            let (x, y) = rope[i].position;
            let (head_x, head_y) = rope[i-1].position;
            let difference_x = head_x - x;
            let difference_y = head_y - y;

            let change = if difference_x.abs() > 1 || difference_y.abs() > 1 {
                (difference_x.signum(), difference_y.signum())
            } else {
                (0, 0)
            };

            rope[i].position.0 += change.0;
            rope[i].position.1 += change.1;
        }

        visited_tail_positions.insert(rope[rope.len() - 1].position);
    }

    return visited_tail_positions.len() as u32;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn solve_test() {
         let input = fs::read_to_string(TEST_PATH).expect("Failed to parse file to string");

         let output = solve(&input);

         assert_eq!(13, output);
    }
    #[test]
    fn parse_test() {
        //Just the 11 initial values for the sake of sanity
        let test_commands 
            = vec![Command::Right,Command::Right,Command::Right,Command::Right,Command::Up,Command::Up,Command::Up,Command::Up,Command::Left,Command::Left,Command::Left];
        
        let input = fs::read_to_string(TEST_PATH).expect("Failed to parse file to string");
        let parsed_commands = parse_string_to_commands(&input);

        let matching = parsed_commands.iter().zip(&test_commands).filter(|&(a, b)| a == b).count();
        assert_eq!(11, matching);
    }
}