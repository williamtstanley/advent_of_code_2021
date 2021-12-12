const INPUT: &'static str = include_str!("../inputs/3.txt");

pub(crate) fn run() {
    println!("Day 3");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let vec_lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut gamma_rate = "".to_string();
    let mut epsilon_rate = "".to_string();
    let len = vec_lines[0].len();

    for i in 0..len {
        let most_common_bit = find_most_common_bit(&vec_lines, i, '0');
        if most_common_bit == '1' {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
        } else {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        }
    }

    let gamma = usize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_rate, 2).unwrap();

    gamma * epsilon
}

fn part2(input: &str) -> usize {
    let vec_lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let oxygen_generator_rating = find_oxygen_generator_rating(&vec_lines);
    let c02_scrubber_rating = find_c02_scrubber_rating(&vec_lines);

    oxygen_generator_rating * c02_scrubber_rating
}

fn find_most_common_bit(lines: &Vec<Vec<char>>, position: usize, tie_breaker: char) -> char {
    let mut count = 0;
    for line in lines {
        if line[position] == '1' {
            count += 1;
        } else {
            count -= 1;
        }
    }

    match count {
        d if d > 0 => '1',
        d if d < 0 => '0',
        _ => tie_breaker,
    }
}

fn find_least_common_bit(lines: &Vec<Vec<char>>, position: usize) -> char {
    let most_common_bit = find_most_common_bit(lines, position, '1');
    if most_common_bit == '1' {
        '0'
    } else {
        '1'
    }
}

fn find_oxygen_generator_rating(lines: &Vec<Vec<char>>) -> usize {
    let mut filtered_lines = lines.clone();
    let mut position = 0;
    while filtered_lines.len() > 1 {
        let most_common_bit = find_most_common_bit(&filtered_lines, position, '1');
        filtered_lines = filtered_lines
            .into_iter()
            .filter(|line| line[position] == most_common_bit)
            .collect();
        position += 1;
    }

    let line = filtered_lines[0].clone();
    let binary_string: String = line.into_iter().collect();
    usize::from_str_radix(&binary_string, 2).unwrap()
}

fn find_c02_scrubber_rating(lines: &Vec<Vec<char>>) -> usize {
    let mut filtered_lines = lines.clone();
    let mut position = 0;
    while filtered_lines.len() > 1 {
        let least_common_bit = find_least_common_bit(&filtered_lines, position);
        filtered_lines = filtered_lines
            .into_iter()
            .filter(|line| line[position] == least_common_bit)
            .collect();
        position += 1;
    }

    let line = filtered_lines[0].clone();
    let binary_string: String = line.into_iter().collect();
    usize::from_str_radix(&binary_string, 2).unwrap()
}
