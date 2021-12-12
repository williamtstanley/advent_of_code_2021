use std::str::FromStr;
const INPUT: &'static str = include_str!("../inputs/2.txt");

pub(crate) fn run() {
    println!("Day 2");
    println!("Part 2: {}", part2(INPUT));
}

fn part2(input: &str) -> usize {
    // position, depth, aim
    let mut position = (0, 0, 0);
    let command_pairs = parse_commands(input.lines());
    for (direction, distance) in command_pairs {
        position = execute_command(direction, distance, &mut position);
    }

    position.0 * position.1
}

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

fn execute_command(
    direction: Command,
    distance: usize,
    position: &mut (usize, usize, usize),
) -> (usize, usize, usize) {
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

fn parse_commands(lines: std::str::Lines) -> Vec<(Command, usize)> {
    let mut command_pairs: Vec<(Command, usize)> = Vec::new();
    for line in lines {
        let mut args = line.split_whitespace();
        let command = Command::from_str(&args.next().unwrap()).unwrap();
        let number = args.next().unwrap().parse::<usize>().unwrap();
        command_pairs.push((command, number));
    }

    command_pairs
}
