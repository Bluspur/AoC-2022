use std::fs;

const INPUT_PATH: &str = "..\\assets\\input.txt";

#[derive(Debug)]
struct Map {
    values: Vec<Vec<u32>>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let values = input
            .lines()
            .map(|line| {
                line
                    .chars()
                    .map(|char|
                        char
                            .to_digit(10)
                            .expect("char should be a valid alphanumeric character")
                        )
                    .collect()
            })
            .collect();
        Map { values }
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || x == self.values[0].len() - 1 || y == 0 || y == self.values.len() - 1 {
            return true;
        }
        let north = self.values[..y].iter().map(|v| v[x]).max().unwrap() < self.values[y][x];
        let south = self.values[y+1..].iter().map(|v| v[x]).max().unwrap() < self.values[y][x];
        let east = self.values[y][x+1..].iter().max().unwrap() < &self.values[y][x];
        let west = self.values[y][..x].iter().max().unwrap() < &self.values[y][x];

        if north|south|east|west {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let input_text = fs::read_to_string(INPUT_PATH).expect("Input file should be readable as a string");
    let answer = solve(&input_text);
    println!("{answer}");
}

fn solve(input: &str) -> u32 {
    let map = Map::from_str(input);
    map.values
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row
                .iter()
                .enumerate()
                .filter(|(x, _)| map.is_visible(*x, y))
                .count() as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_PATH: &str = "..\\assets\\test.txt";


    #[test]
    fn solve_test() {
        let input_text = fs::read_to_string(TEST_PATH).expect("Input file should be readable as a string");
        let answer = solve(&input_text);
        assert_eq!(21, answer);
    }
    #[test]
    fn parse_test() {
        let input_text = fs::read_to_string(TEST_PATH).expect("Input file should be readable as a string");
        let reference_answer = Map{values: vec![vec![3,0,3,7,3],vec![2,5,5,1,2],vec![6,5,3,3,2],vec![3,3,5,4,9],vec![3,5,3,9,0]] };
        let parsed_input = Map::from_str(&input_text);
        assert_eq!(reference_answer.values, parsed_input.values);
    }
}