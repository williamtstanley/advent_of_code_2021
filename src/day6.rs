const INPUT: &'static str = include_str!("../inputs/6.txt");

pub(crate) fn run() {
    println!("Day 6");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    // This implementation was a disaster when the total days count went beyond 100
    let mut school = School::new(
        input
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .into_iter()
            .map(Lanternfish::new)
            .collect(),
    );

    for _ in 0..80 {
        school.tick();
    }

    school.lanternfish.iter().count() as i32
}

fn part2(input: &str) -> usize {
    let mut buckets = Buckets::new();
    for number in input.split(",") {
        buckets.insert(number.parse().unwrap());
    }

    for _ in 0..256 {
        buckets.step_day();
    }

    buckets.count()
}

#[derive(Debug)]
struct Buckets {
    buckets: [usize; 9],
}
impl Buckets {
    fn new() -> Self {
        Self { buckets: [0; 9] }
    }
    fn insert(&mut self, index: usize) {
        self.buckets[index] += 1;
    }

    fn step_day(&mut self) {
        self.buckets.rotate_left(1);
        self.buckets[6] += self.buckets[8];
    }

    fn count(&self) -> usize {
        self.buckets.into_iter().sum()
    }
}

#[derive(Debug, Clone)]
struct Lanternfish {
    interval_count: i32,
    ready: bool,
}

impl Lanternfish {
    fn new(count: i32) -> Lanternfish {
        Lanternfish {
            interval_count: count,
            ready: false,
        }
    }

    fn tick(&mut self) {
        self.interval_count -= 1;
        if self.interval_count < 0 {
            self.interval_count = 6;
            self.ready = true;
        }
    }

    fn spawn_new(&mut self) -> Option<Lanternfish> {
        if self.ready {
            self.ready = false;
            return Some(Lanternfish::new(8));
        }
        None
    }
}

#[derive(Debug, Clone)]
struct School {
    lanternfish: Vec<Lanternfish>,
}

impl School {
    fn new(lanternfish: Vec<Lanternfish>) -> School {
        School {
            lanternfish: lanternfish,
        }
    }
    fn tick(&mut self) {
        let mut new_lanternfish = vec![];
        for fish in &mut self.lanternfish {
            fish.tick();
            if let Some(new_fish) = fish.spawn_new() {
                new_lanternfish.push(new_fish);
            }
        }
        self.lanternfish.append(&mut new_lanternfish);
    }
}
