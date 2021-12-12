const INPUT: &'static str = include_str!("../inputs/7.txt");

pub(crate) fn run() {
    println!("Day 7");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let mut numbers = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    numbers.sort();
    let median_position = numbers.len() / 2;
    let median = numbers[median_position];

    let distance: i32 = numbers.into_iter().map(|n| (median - n).abs()).sum();

    distance as usize
}

fn part2(input: &str) -> usize {
    let numbers: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let min = numbers.iter().min().unwrap();
    let max = numbers.iter().max().unwrap();

    // I am sure there is a maths way to do this, but I don't know it.
    // brute force it. Calculate all possible costs for all possitions and
    // find the lowest.
    let min_fuel: i32 = (*min..=*max)
        .into_iter()
        .map(|possible_position| {
            numbers
                .iter()
                .map(|current_position| {
                    let distance = (possible_position - current_position).abs();
                    (1 + distance) * distance / 2
                })
                .sum()
        })
        .min()
        .unwrap();

    min_fuel as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn first() {
        assert_eq!(part1(INPUT), 37);
    }

    #[test]
    fn second() {
        assert_eq!(part2(INPUT), 168);
    }
}
