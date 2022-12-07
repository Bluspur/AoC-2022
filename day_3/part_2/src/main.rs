use std::{fs, collections::HashSet, io::BufRead};
use regex::Regex;

fn main() {
    let input = fs::read_to_string("..\\assets\\input.txt").expect("Could not Parse File");
    let total_score = solve(&input);
    println!("Priority Total Sum = {}", total_score);
}

fn solve(input: &str) -> u32 {
    let group_seperator = Regex::new(r"((?m:^.+\s){3})").expect("Invalid regex");
    
    let answer: u32 = group_seperator
        .captures_iter(input)
        .map(|capture| {
            let group_inventory = &capture[0].trim();
            let mut inventories = group_inventory.lines();

            find_common_item(&mut inventories)
        })
        .map(|item| get_item_priority(item))
        .sum();

    answer
}

fn find_common_item(iter: &mut dyn Iterator<Item = &str>) -> char {
    let iter_sets: Vec<HashSet<char>> = iter
        .map(|items| HashSet::<char>::from_iter(items.chars()))
        .collect();

    iter_sets.as_slice()[1..]
        .iter()
        .fold(iter_sets.first().unwrap().clone(), |intersection, set| {
            intersection.intersection(set).copied().collect()
        })
        .iter()
        .next()
        .unwrap()
        .clone()
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

        assert_eq!(70, solve(&input));
    }
}