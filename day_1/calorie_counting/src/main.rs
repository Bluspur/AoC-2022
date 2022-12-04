use std::fs;

fn main() {
    let file = fs::read_to_string("..\\assets\\input.txt").expect("Could not Parse File");
    let lines = file.lines();
    let mut highest_value = 0;
    let mut current_value = 0;
    for line in lines {
        match line.parse::<i32>() {
            Ok(val) => {
                current_value += val;
            }
            Err(_) => {
                if current_value > highest_value {
                    highest_value = current_value;
                }
                current_value = 0; 
            }
        }
    }
    println!("{}",highest_value);
}
