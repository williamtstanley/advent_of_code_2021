const INPUT: &'static str = include_str!("../inputs/4.txt");

pub(crate) fn run() {
    println!("Day 4");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let (drawn_numbers, mut boards) = drawn_numbers_and_boards(input);
    for n in drawn_numbers {
        for board in boards.iter_mut() {
            board.mark_match(n);
        }
    }

    let mut score: usize = 0;
    // first winner
    let winner = boards.iter().min_by_key(|b| b.calls_count);
    if let Some(winner) = winner {
        let sum = winner.get_sum();
        score = sum * winner.winning_number
    }

    score
}

fn part2(input: &str) -> usize {
    let (drawn_numbers, mut boards) = drawn_numbers_and_boards(input);
    for n in drawn_numbers {
        for board in boards.iter_mut() {
            board.mark_match(n);
        }
    }

    let mut score: usize = 0;
    // // last winner
    let last_winner = boards.iter().max_by_key(|b| b.calls_count);
    if let Some(last_winner) = last_winner {
        score = last_winner.get_sum() * last_winner.winning_number
    }

    score
}

#[derive(Debug, Clone, Copy)]
struct Square(usize, bool);

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Square>,
    calls_count: usize,
    is_winner: bool,
    winning_number: usize,
}

impl Board {
    fn new(grid: Vec<Square>) -> Board {
        Board {
            grid,
            calls_count: 0,
            is_winner: false,
            winning_number: 0,
        }
    }

    fn mark_match(&mut self, n: usize) {
        if !self.is_winner {
            self.calls_count += 1;
            self.grid = self
                .grid
                .clone()
                .into_iter()
                .map(|square| {
                    if square.0 == n {
                        Square(square.0, true)
                    } else {
                        square
                    }
                })
                .collect();

            self.is_winner = self.set_winner();
            if self.is_winner {
                self.winning_number = n;
            }
        }
    }

    fn set_winner(&self) -> bool {
        // for each possible winning state check the grid indexes for matches and return true if all indexes match
        let mut matches = 0;
        for winning_state in BOARD_WIN_STATES.iter() {
            for i in winning_state {
                if self.grid[*i as usize].1 {
                    matches += 1;
                }
            }
            if matches == 5 {
                return true;
            }
            matches = 0;
        }

        false
    }

    fn get_sum(&self) -> usize {
        self.grid
            .iter()
            .fold(0, |acc, s| if s.1 { acc } else { acc + s.0 })
    }
}

const BOARD_WIN_STATES: [[usize; 5]; 10] = [
    [0, 1, 2, 3, 4],
    [5, 6, 7, 8, 9],
    [10, 11, 12, 13, 14],
    [15, 16, 17, 18, 19],
    [20, 21, 22, 23, 24],
    [0, 5, 10, 15, 20],
    [1, 6, 11, 16, 21],
    [2, 7, 12, 17, 22],
    [3, 8, 13, 18, 23],
    [4, 9, 14, 19, 24],
];

fn drawn_numbers_and_boards(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut drawn_numbers = Vec::new();
    let mut board_lines = Vec::new();
    let mut boards = Vec::new();
    for (index, line) in input.lines().enumerate() {
        if index == 0 {
            for n in line.split(",") {
                drawn_numbers.push(n.parse::<usize>().unwrap());
            }
        }
        if index > 0 && !line.is_empty() {
            board_lines.push(line);
            if board_lines.len() == 5 {
                let mut grid = Vec::new();
                for board_line in board_lines.iter() {
                    for n in board_line.split_whitespace() {
                        grid.push(Square(n.parse::<usize>().unwrap(), false));
                    }
                }
                boards.push(Board::new(grid));
                board_lines.clear();
            }
        }
    }

    (drawn_numbers, boards)
}
