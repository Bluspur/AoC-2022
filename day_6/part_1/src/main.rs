use std::{fs, collections::HashSet};

fn main() {
    let file = fs::read_to_string("..\\assets\\input.txt").expect("Could not Parse File");
    let outcome = solve(&file);
    match outcome {
        Some(index) => println!("First Index: {}", index),
        None => println!("No match found"),
    }
}

fn solve(input: &str) -> Option<i32> {
    let input = input.trim().as_bytes();
    'outer: for index in 3..input.len() {
        let mut buffer: HashSet<u8> = HashSet::new();
        for i in index - 3..=index {
            if !buffer.insert(input[i]) {
                continue 'outer;
            }
        }
        return Some(index as i32 + 1);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let file = fs::read_to_string("..\\assets\\test.txt").expect("Could not Parse File");
        let output = solve(&file);
        assert_ne!(None, output);
        assert_eq!(7, output.unwrap());
    }
}
