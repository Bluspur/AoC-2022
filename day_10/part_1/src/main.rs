use std::fs;

const INPUT_PATH: &str = "..\\assets\\input.txt";
const TEST_PATH: &str = "..\\assets\\test.txt";

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH)
        .expect("Failed to parse input file to string");
    let output = solve(&input);
    println!("Output: {}", output);
}

fn solve(input: &str) -> i32 {
    let instructions = parse_input(input);

    let mut x = 1;
    let mut cycle = 1;
    let mut total = 0;

    for instruction in instructions {
        if cycle % 40 == 20 {
            total += cycle * x;
        }

        cycle += 1;

        if let Instruction::Addx(value) = instruction {
            if cycle % 40 == 20 {
                total += cycle * x;
            }
            
            x += value;
            cycle += 1;
        }
    }

    total
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let segments = line.split_once(' ');
            match segments {
                Some((_, v)) => Instruction::Addx(v.parse::<i32>().unwrap()),
                None => Instruction::Noop,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_test() {
        let input = fs::read_to_string(TEST_PATH).expect("Failed to parse input file to string");
        let output = solve(&input);
        assert_eq!(13140, output);
    }

    #[test]
    fn parse_test() {
        let input = fs::read_to_string(TEST_PATH).expect("Failed to parse input file to string");
        let parsed_output: Vec<Instruction> = parse_input(&input);

        assert_eq!(Instruction::Addx(15), parsed_output[0]);
        assert_eq!(Instruction::Addx(-11), parsed_output[1]);
        assert_eq!(Instruction::Addx(6), parsed_output[2]);
        assert_eq!(Instruction::Addx(-3), parsed_output[3]);

        assert_eq!(Instruction::Noop, parsed_output[9]);
    }
}