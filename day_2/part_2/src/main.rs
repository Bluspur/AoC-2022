use std::fs;

fn main() {
    let input = fs::read_to_string("..\\assets\\input.txt").expect("Could not Parse File");
    let total_score = calculate_total_score(&input);
    println!("Final Score = {}", total_score);
}

fn calculate_total_score(input: &str) -> i32 {
    let input = input.lines();
    let mut total = 0;
    for round in input {
        let score = calculate_round_score(round);
        total += score;
    }
    total
}

fn calculate_round_score(match_info:&str) -> i32 {
    let (opponent_shape, player_shape) = match_string_to_shape(match_info);    
    let mut score = player_shape as i32;
    let match_result_score = match (player_shape, opponent_shape) {
        (HandShape::Rock, HandShape::Scissors) | (HandShape::Paper, HandShape::Rock) | (HandShape::Scissors, HandShape::Paper) => 6,
        (HandShape::Rock, HandShape::Rock) | (HandShape::Paper, HandShape::Paper) | (HandShape::Scissors, HandShape::Scissors) => 3,
        _ => 0,
    };
    score += match_result_score;
    score
}

fn match_string_to_shape(match_string:&str) -> (HandShape,HandShape) {
    let opponent_shape = match_string.chars().next().unwrap();
    let player_shape = match_string.chars().last().unwrap();
    
    (HandShape::from_char(opponent_shape), HandShape::from_char(player_shape))
}

#[derive(PartialEq, Copy, Clone)]
enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl HandShape {
    fn from_char(shape_char: char) -> Self {
    match shape_char {
        'A'|'X' => HandShape::Rock,
        'B'|'Y' => HandShape::Paper,
        'C'|'Z' => HandShape::Scissors,
        _ => panic!("Invalid char"),
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strategy_guide() {
        let input = fs::read_to_string("..\\assets\\test.txt").expect("Could not Parse File");
        
        assert_eq!(15, calculate_total_score(&input));
    }
}