use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    if let Ok(lines) = read_lines("./input.txt") {
        let vec_lines: Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();
        // let mut gamma_rate = "".to_string();
        // let mut epsilon_rate = "".to_string();
        // let len = vec_lines[0].len();

        // for i in 0..len {
        //     let most_common_bit = find_most_common_bit(&vec_lines, i, '0');
        //     if most_common_bit == '1' {
        //         gamma_rate.push_str("1");
        //         epsilon_rate.push_str("0");
        //     } else {
        //         gamma_rate.push_str("0");
        //         epsilon_rate.push_str("1");
        //     }
        // }

        // let gamma = isize::from_str_radix(&gamma_rate, 2).unwrap();
        // let epsilon = isize::from_str_radix(&epsilon_rate, 2).unwrap();

        // println!("g: {}, e: {}, p: {}", gamma, epsilon, gamma * epsilon);
        let oxygen_generator_rating = find_oxygen_generator_rating(&vec_lines);
        let c02_scrubber_rating = find_c02_scrubber_rating(&vec_lines);
        println!(
            "og: {:?}, c02: {:?}, p: {:?}",
            oxygen_generator_rating,
            c02_scrubber_rating,
            oxygen_generator_rating * c02_scrubber_rating
        );
    } else {
        println!("Could not read file");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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

fn find_oxygen_generator_rating(lines: &Vec<Vec<char>>) -> isize {
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
    isize::from_str_radix(&binary_string, 2).unwrap()
}

fn find_c02_scrubber_rating(lines: &Vec<Vec<char>>) -> isize {
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
    isize::from_str_radix(&binary_string, 2).unwrap()
}
