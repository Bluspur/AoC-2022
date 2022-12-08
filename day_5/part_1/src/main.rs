use std::fs;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("..\\assets\\input.txt").expect("Could not Parse File");
    let word_combo = solve(&file);
    println!("{word_combo}");
}

fn solve(input: &str) -> String {
    let (setup, instructions) = input.split_once("\n\n").unwrap();
    let mut stack_map = parse_setup(setup);

    for instruction in parse_instructions(instructions) {
        for _ in 0..instruction.quantity {
            let item_crate = stack_map[instruction.take_from - 1].pop().unwrap();
            stack_map[instruction.deliver_to - 1].push(item_crate);
        }
    }

    let mut output = String::new();
    for mut stack in stack_map {
        output.push(stack.pop().unwrap());
    }
    return output;
}

fn parse_setup(setup: &str) -> Vec<Vec<char>> {
    let (stacks, platforms) = setup.rsplit_once("\n").unwrap();
    let platform_count = platforms.chars().filter(|c| !c.is_whitespace()).count();
    let mut map = vec![Vec::new(); platform_count];
    for line in stacks.lines().rev() {
        let mut line_index = 1;
        for i in 0..platform_count {
            let crate_id = line.chars().nth(line_index).unwrap();
            if crate_id.is_alphabetic() {
                map[i].push(crate_id);
            }
            line_index += 4;
        } 
    }
    return map;
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    let mut instruction_set = Vec::new();
    let pattern = Regex::new(r"move (?P<quantity>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    for caps in pattern.captures_iter(instructions) {
        let quantity = caps["quantity"].parse::<i32>().unwrap();
        let take_from = caps["from"].parse::<usize>().unwrap();
        let deliver_to = caps["to"].parse::<usize>().unwrap();
        let instruction = Instruction {
            quantity,
            take_from,
            deliver_to,
        };
        instruction_set.push(instruction);
    }
    return instruction_set;
}

struct Instruction {
    quantity: i32,
    take_from: usize,
    deliver_to: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let input = fs::read_to_string("..\\assets\\test.txt").expect("Could not Parse File");

        assert_eq!("CMZ".to_string(), solve(&input));
    }

    #[test]
    fn parse_instructions_test() {
        let input = fs::read_to_string("..\\assets\\test.txt").expect("Could not Parse File");
        let (_, instructions) = input.split_once("\r\n\r\n").unwrap();
        let instruction_set = parse_instructions(instructions);

        assert_eq!(1, instruction_set[0].quantity);
        assert_eq!(2, instruction_set[0].take_from);
        assert_eq!(1, instruction_set[0].deliver_to);

        assert_eq!(3, instruction_set[1].quantity);
        assert_eq!(1, instruction_set[1].take_from);
        assert_eq!(3, instruction_set[1].deliver_to);

        assert_eq!(2, instruction_set[2].quantity);
        assert_eq!(2, instruction_set[2].take_from);
        assert_eq!(1, instruction_set[2].deliver_to);

        assert_eq!(1, instruction_set[3].quantity);
        assert_eq!(1, instruction_set[3].take_from);
        assert_eq!(2, instruction_set[3].deliver_to);
    }
}