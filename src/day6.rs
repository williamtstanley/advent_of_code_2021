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
    // keep track of the number of fish in each interval position
    let mut buckets = [0; 9];

    // for each initial fish interval update the count at that index
    for number in input.split(",") {
        let number = number.parse::<usize>().unwrap();
        buckets[number] += 1;
    }

    // for each day move each fish to the next interval
    // for the fish that wrap they "breed" so add one for each at index 6
    for _ in 0..256 {
        buckets.rotate_left(1);
        buckets[6] += buckets[8];
    }

    buckets.into_iter().sum()
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
