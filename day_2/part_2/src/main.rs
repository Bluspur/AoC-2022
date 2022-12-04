use std::fs;

fn main() {
    let input = fs::read_to_string("..\\assets\\input.txt").expect("Could not Parse File");
    let total_score = solve(&input);
    println!("Final Score = {}", total_score);
}

fn solve(input: &str) -> i32 {
    let mut total_score = 0;
    for round in input.lines() {
        let (opponent_shape, round_outcome) = parse_round_info(round);
        let player_shape = match round_outcome {
            RoundOutcome::Win => opponent_shape.get_victor(),
            RoundOutcome::Draw => opponent_shape,
            RoundOutcome::Lose => opponent_shape.get_loser(),
        };
        total_score += round_outcome as i32 + player_shape as i32;
    }
    return total_score
}

fn parse_round_info(input: &str) -> (HandShape, RoundOutcome) {
    let opponent_shape = input.chars().nth(0).unwrap();
    let outcome = input.chars().nth(2).unwrap();
    (HandShape::from_char(opponent_shape),RoundOutcome::from_char(outcome))
}

#[derive(PartialEq, Clone, Copy)]
enum RoundOutcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}
impl RoundOutcome {
    fn from_char(outcome_char: char) -> Self {
        match outcome_char {
            'X' => RoundOutcome::Lose,
            'Y' => RoundOutcome::Draw,
            'Z' => RoundOutcome::Win,
            n => panic!("Unexpected conversion from char {n} to outcome"),
        }
    }
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
        'A' => HandShape::Rock,
        'B' => HandShape::Paper,
        'C' => HandShape::Scissors,
         n => panic!("Unexpected conversion from char {n} to shape"),
        }
    }
    fn get_victor(&self) -> HandShape {
        match self {
            HandShape::Rock => HandShape::Paper,
            HandShape::Paper => HandShape::Scissors,
            HandShape::Scissors => HandShape::Rock,
        }
    }
    fn get_loser(&self) -> HandShape {
        self.get_victor().get_victor()
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        let input = fs::read_to_string("..\\assets\\test.txt").expect("Could not Parse File");
        
        assert_eq!(12, solve(&input));
    }
}