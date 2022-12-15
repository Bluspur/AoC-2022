use std::fs;

const INPUT_PATH: &str = "..\\assets\\input.txt";
const TEST_PATH: &str = "..\\assets\\test.txt";

fn main() {
    let input = fs::read_to_string(INPUT_PATH).expect("Failed to parse input file to string");
    let output = solve(&input);
    println!("Output: {}", output);
}

fn solve(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_test() {
        let input = fs::read_to_string(TEST_PATH).expect("Failed to parse input file to string");
        let output = solve(&input);
        assert_eq!(0, output);
    }
}