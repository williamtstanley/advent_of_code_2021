use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone)]
struct Line {
    points: Vec<Point>,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line {
            points: all_points(start, end),
        }
    }
}
fn all_points(start: Point, end: Point) -> Vec<Point> {
    let mut xs: Vec<i32> = Vec::new();
    let mut ys: Vec<i32> = Vec::new();
    let sy: i32 = start.y;
    let ey: i32 = end.y;
    let sx: i32 = start.x;
    let ex: i32 = end.x;

    let x_direction = match sx {
        x if x > ex => -1,
        x if x < ex => 1,
        _ => 0,
    };

    let y_direction = match sy {
        y if y > ey => -1,
        y if y < ey => 1,
        _ => 0,
    };

    let dif = cmp::max((sx - ex).abs(), (sy - ey).abs());
    for i in 0..=dif {
        xs.push(sx + i * x_direction);
        ys.push(sy + i * y_direction);
    }

    xs.iter()
        .zip(ys.iter())
        .map(|(x, y)| Point { x: *x, y: *y })
        .collect()
}

fn main() {
    let count: usize = read_lines("input.txt")
        .iter()
        .flat_map(|line| line_from_string(line).points)
        .fold(HashMap::new(), |mut map, point| {
            *map.entry(point).or_insert(0) += 1;
            map
        })
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .count();

    println!("counts: {:?}", count);
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn line_from_string(line: &str) -> Line {
    let points: Vec<Point> = line
        .split(" -> ")
        .map(|s| {
            let coords = s
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Point {
                x: coords[0],
                y: coords[1],
            }
        })
        .collect();

    Line::new(points[0], points[1])
}
