use std::fs::File;
use std::io::{BufRead, self};

#[derive(Debug)]
struct Command {
    action: String,
    units: i32
}

fn main() {
    let commands = get_commands("inputs/day02.txt");
    println!("result (part1): {}", follow_instructions(&commands));
    println!("result (part2): {}", follow_instructions_with_aim(&commands));
}

fn follow_instructions(commands: &Vec<Command>) -> i32 {
    let mut h_pos = 0;
    let mut d_pos = 0;

    for c in commands {
        match c.action.as_str() {
            "forward" => h_pos += c.units,
            "up" => d_pos -= c.units,
            "down" => d_pos += c.units,
            _ => println!("invalid action command")
        }
    }
    h_pos * d_pos
}

fn follow_instructions_with_aim(commands: &Vec<Command>) -> i32 {
    let mut h_pos = 0;
    let mut d_pos = 0;
    let mut aim = 0;

    for c in commands {
        match c.action.as_str() {
            "forward" => {h_pos += c.units; d_pos += aim * c.units },
            "up" => aim -= c.units,
            "down" => aim += c.units,
            _ => println!("invalid action command")
        }
    }
    h_pos * d_pos
}


fn get_commands(input: &str) -> Vec<Command> {
    let file = File::open(input).unwrap();
    let commands: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().split(" ").map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|v| Command{action: v[0].clone(), units: v[1].parse::<i32>().unwrap()})
        .collect::<Vec<_>>();

    commands
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let c = vec![
            Command{action: String::from("forward"), units: 5},
            Command{action: String::from("down"), units: 5},
            Command{action: String::from("forward"), units: 8},
            Command{action: String::from("up"), units: 3},
            Command{action: String::from("down"), units: 8},
            Command{action: String::from("forward"), units: 2},
        ];
        assert_eq!(follow_instructions(&c), 150)
    }

    #[test]
    fn test_part2() {
        let c = vec![
            Command{action: String::from("forward"), units: 5},
            Command{action: String::from("down"), units: 5},
            Command{action: String::from("forward"), units: 8},
            Command{action: String::from("up"), units: 3},
            Command{action: String::from("down"), units: 8},
            Command{action: String::from("forward"), units: 2},
        ];
        assert_eq!(follow_instructions_with_aim(&c), 900)
    }
}
