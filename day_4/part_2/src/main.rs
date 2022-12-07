use std::{fs, ops::RangeInclusive};

fn main() {
    let file = fs::read_to_string("..\\assets\\input.txt").expect("Could not Parse File");
    let total_score = solve(&file);
    println!("{total_score}");
}

fn solve(input: &str) -> i32 {
    let mut overlap_count = 0;
    for line in input.lines() {        
        let (left_elf, right_elf) = line.split_once(',').unwrap();
        let left_elf = parse_sequence(left_elf);
        let right_elf = parse_sequence(right_elf);
        overlap_count += is_overlapping(left_elf, right_elf) as i32;
    }
    overlap_count
}

fn parse_sequence(input: &str) -> RangeInclusive<i32> {
    let mut output = input.trim().split('-').map(|c| c.parse::<i32>().unwrap()).take(2);
    let lower_bound = output.next().unwrap();
    let upper_bound = output.next().unwrap();
    lower_bound..=upper_bound
}

fn is_overlapping(input_1: RangeInclusive<i32>, input_2: RangeInclusive<i32>) -> bool {
    input_1.start() <= input_2.end() && input_2.start() <= input_1.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let input = fs::read_to_string("..\\assets\\test.txt").expect("Could not Parse File");

        assert_eq!(4, solve(&input));
    }

    #[test]
    fn parse_sequence_test() {
        let input = "7-24";
        let test_output = parse_sequence(input);
        assert_eq!(&7, test_output.start());
        assert_eq!(&24, test_output.end());
    }

    #[test]
    fn overlapping_test() {
        let input_1 = "2-8";
        let input_2 = "3-7";
        let range_1 = parse_sequence(input_1);
        let range_2 = parse_sequence(input_2);
        assert!(is_overlapping(range_1, range_2));
    }
}