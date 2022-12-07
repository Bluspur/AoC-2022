use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("..\\assets\\input.txt").expect("Could not Parse File");
    let total_score = solve(&input);
    println!("Priority Total Sum = {}", total_score);
}

fn solve(input: &str) -> u32 {
    //identify common elements in compartments
    //return the priority value
    let rucksacks = input.lines();
    let mut rucksack_priority_score = 0;
    for rucksack in rucksacks {
        let compartment_border = rucksack.chars().count() / 2;
        let left_compartment = &rucksack[..compartment_border];
        let right_compartment = &rucksack[compartment_border..];
        let common_characters = left_compartment.chars().filter(|&c|right_compartment.chars().any(|x| x == c)).unique();
        let priority_score: u32 = common_characters.map(|c| get_item_priority(c)).sum();
        rucksack_priority_score += priority_score;
    }
    rucksack_priority_score
}

fn get_item_priority(id: char) -> u32 {
    if !id.is_alphabetic() {
        panic!("{} is not an alphabetic character", id);
    }
    if id.is_lowercase() {
        id as u32 -'a' as u32 + 1
    } else {
        id as u32 -'A' as u32 + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let input = fs::read_to_string("..\\assets\\test.txt").expect("Could not Parse File");

        assert_eq!(157, solve(&input));
    }
}