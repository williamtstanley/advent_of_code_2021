use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut line_numbers: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                line_numbers.push(ip.parse::<i32>().unwrap());
            }
        }

        let window_vec = calculate_windows(line_numbers);

        println!("{:?}", count_window_increases(window_vec));
    }
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// EXERCISE 1
// fn count_increases(lines: Vec<i32>) -> i32 {
//     let mut increase_count: i32 = 0;
//     let mut first_line: bool = true;
//     let mut previous_line: i32 = 0;
//     for line_num in lines {
//         if first_line {
//             previous_line = line_num;
//             first_line = false;
//         } else {
//             if line_num > previous_line {
//                 increase_count += 1;
//             }
//             previous_line = line_num;
//         }
//     }
//     increase_count
// }
