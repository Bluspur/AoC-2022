use std::fs;

const INPUT_PATH: &str = "..\\assets\\input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_PATH)
        .expect("Failed to parse input file to string");
    let output = solve(&input);
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