use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Command {
    Down,
    Up,
    Forward,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "down" => Ok(Command::Down),
            "up" => Ok(Command::Up),
            "forward" => Ok(Command::Forward),
            _ => Err(()),
        }
    }
}

fn main() {
    // position, depth, aim
    let mut position = (0, 0, 0);
    let command_pairs = parse_commands(read_lines("input.txt"));
    for (direction, distance) in command_pairs {
        position = execute_command(direction, distance, &mut position);
    }
    println!("{:?}", position);
}

fn execute_command(
    direction: Command,
    distance: i32,
    position: &mut (i32, i32, i32),
) -> (i32, i32, i32) {
    match direction {
        Command::Down => position.2 += distance,
        Command::Up => position.2 -= distance,
        Command::Forward => {
            position.0 += distance;
            position.1 += distance * position.2;
        }
    }
    position.clone()
}

fn parse_commands(lines: io::Result<io::Lines<io::BufReader<File>>>) -> Vec<(Command, i32)> {
    let mut command_pairs: Vec<(Command, i32)> = Vec::new();
    if let Ok(lines) = lines {
        for line in lines {
            if let Ok(ip) = line {
                let mut args = ip.split_whitespace();
                let command = Command::from_str(&args.next().unwrap()).unwrap();
                let number = args.next().unwrap().parse::<i32>().unwrap();
                command_pairs.push((command, number));
            }
        }
    }

    command_pairs
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
