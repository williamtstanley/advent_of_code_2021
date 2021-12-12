const INPUT: &'static str = include_str!("../inputs/1.txt");

pub(crate) fn run() {
    println!("Day 1");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut increase_count: usize = 0;
    let mut first_line: bool = true;
    let mut previous_line: usize = 0;
    for line_num in lines {
        if first_line {
            previous_line = line_num;
            first_line = false;
        } else {
            if line_num > previous_line {
                increase_count += 1;
            }
            previous_line = line_num;
        }
    }

    increase_count
}

fn part2(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let window_vec = calculate_windows(lines);

    count_window_increases(window_vec)
}

fn calculate_windows(lines: Vec<i32>) -> Vec<(i32, i32, i32)> {
    let mut windows: Vec<(i32, i32, i32)> = Vec::new();

    for i in 0..lines.len() - 2 {
        let n1 = lines[i];
        let n2 = lines[i + 1];
        let n3 = lines[i + 2];
        windows.push((n1, n2, n3));
    }

    windows
}

fn sum_window(window: (i32, i32, i32)) -> i32 {
    let (n1, n2, n3) = window;
    n1 + n2 + n3
}

// EXERCISE 2
fn count_window_increases(windows: Vec<(i32, i32, i32)>) -> i32 {
    let mut increase_count: i32 = 0;
    for i in 0..windows.len() - 1 {
        let window1 = windows[i];
        let window2 = windows[i + 1];
        if sum_window(window2) > sum_window(window1) {
            increase_count += 1;
        }
    }
    increase_count
}
