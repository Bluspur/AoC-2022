use std::fs;
use std::collections::BTreeMap;

const TOTAL_SPACE: u32 = 70_000_000;
const REQUIRED_SPACE: u32 = 30_000_000;

fn main() {
    let input_text = fs::read_to_string("..\\assets\\input.txt").unwrap();
    let total = solve(&input_text);
    println!("{total}");
}

fn solve(input: &str) -> u32 {
    let output = parse_commands(input);
    let tree = build_tree_from_commands(output);
    let free_space = TOTAL_SPACE - tree.get("/").unwrap();
    let minimum_space_to_clear = REQUIRED_SPACE - free_space;
    
    return *tree
        .iter()
        .filter(|e| e.1 >= &minimum_space_to_clear)
        .map(|e| e.1)
        .min()
        .unwrap();
}

fn parse_commands(raw_commands: &str) -> Vec<Command> {
    let mut parsed_commands = Vec::new();
    let raw_commands = raw_commands.split('$').skip(1); //First iterator turns out as a blank line if we don't skip
    for raw_command in raw_commands {
        let (command, remainder) = raw_command.trim().split_at(2);
        if command == "cd" {
            let parsed_command = Command::ChangeDirectory(parse_cd(remainder));
            parsed_commands.push(parsed_command);
        } else if command == "ls" {
            let mut output = Vec::new();
            for line in remainder.trim().lines() {
                output.push(Data::from_string(line));
            }
            parsed_commands.push(Command::List(output));
        } else {
            panic!("Failed to parse command. Unexpected command: {command}");
        }
    }
    parsed_commands
}

fn parse_cd(input: &str) -> Cd {
    match input.trim() {
        "/" => Cd::Root,
        ".." => Cd::Up,
        path => Cd::Down(path.to_owned()),
    }
}

fn build_tree_from_commands(commands: Vec<Command>) -> BTreeMap<String, u32> {
    let root = String::new();
    let mut context: BTreeMap<String, u32> = BTreeMap::new();
    let mut current_directory:Vec<String> = vec![root];
    
    for command in commands {
        match command {
            Command::ChangeDirectory(cd) => {
                match cd {
                    Cd::Root => current_directory.truncate(1),
                    Cd::Up => _ = current_directory.pop(),
                    Cd::Down(path) => current_directory.push(path),
                }
            },
            Command::List(data) => {
                let total_size: u32 = data.iter().map(|a| a.size).sum();
                calculate_new_size(&current_directory,&mut context, total_size);
            },
        }
    }
    
    context
}

fn directory_to_string(directory: &Vec<String>) -> String {
    return directory.iter().fold(String::new(), |acc, x| acc + "/" + x);
}

fn calculate_new_size(directory: &Vec<String>, tree: &mut BTreeMap<String, u32>, size: u32) {
    for i in (0..directory.len()).rev() {
        let path = directory_to_string(&directory[0..=i].to_vec());
        let entry = tree.entry(path).or_insert(0);
        *entry += size;        
    }
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(Cd),
    List(Vec<Data>),
}

#[derive(Debug)]
enum Cd {
    Root,
    Up,
    Down(String)
}

#[derive(Debug)]
struct Data {
    size: u32,
}

impl Data {
    fn new(size: Option<u32>) -> Self {
        Data {
            size: size.unwrap_or(0),
        }
    }
    
    fn from_string(input: &str) -> Data {
        let input = input.trim();
        let (head, _) = input.split_once(' ').expect(format!("Failed to parse ls output: {input}").as_str());
        if head == "dir" {
            return Data::new(None)
        } else {
            let size: u32 = head.parse().expect(format!("Failed to parse {} to u32", head).as_str());
            return Data::new(Some(size))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        let input_text = fs::read_to_string("..\\assets\\test.txt").unwrap();
        let output = solve(&input_text);
        assert_eq!(95437, output);
    }
    #[test]
    fn parse_commands_test() {
        let input_text = fs::read_to_string("..\\assets\\test.txt").unwrap();
        let commands = parse_commands(&input_text);
        
        let count = commands.iter().count(); 
        
        assert_eq!(10, count);       
    }   
}