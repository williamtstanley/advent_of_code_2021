use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct Square(i32, bool);

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Square>,
    calls_count: i32,
    is_winner: bool,
    winning_number: i32,
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

    fn mark_match(&mut self, n: i32) {
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

    fn set_winner(&mut self) -> bool {
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

    fn get_sum(&self) -> i32 {
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

fn main() {
    let filename = "./input.txt";
    let (drawn_numbers, mut boards) = drawn_numbers_and_boards(filename);
    for n in drawn_numbers {
        for board in boards.iter_mut() {
            board.mark_match(n);
        }
        // let winner = boards.iter().find(|board| board.is_winner);
        // if winner.is_some() {
        //     let sum = winner.unwrap().get_sum();
        //     println!(
        //         "WINNER!! winning_number: {}, sum: {}, p: {}",
        //         n,
        //         sum,
        //         sum * n
        //     );
        //     break;
        // }
    }

    let mut last_winner_index = 0;
    let mut max_calls = 0;
    let mut max_calls2 = 0;
    for (index, board) in boards.iter().enumerate() {
        if board.is_winner && board.calls_count > max_calls {
            max_calls = board.calls_count;
            last_winner_index = index;
        }
    }

    let last_winner = &boards[last_winner_index];
    println!(
        "last_winner!! winning_number: {}, sum: {}, p: {}",
        last_winner.winning_number,
        last_winner.get_sum(),
        last_winner.get_sum() * last_winner.winning_number,
    );
}

fn drawn_numbers_and_boards(filename: &str) -> (Vec<i32>, Vec<Board>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut drawn_numbers = Vec::new();
    let mut board_lines = Vec::new();
    let mut boards = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if index == 0 {
            for n in line.split(",") {
                drawn_numbers.push(n.parse::<i32>().unwrap());
            }
        }
        if index > 0 && !line.is_empty() {
            board_lines.push(line);
            if board_lines.len() == 5 {
                let mut grid = Vec::new();
                for board_line in board_lines.iter() {
                    for n in board_line.split_whitespace() {
                        grid.push(Square(n.parse::<i32>().unwrap(), false));
                    }
                }
                boards.push(Board::new(grid));
                board_lines.clear();
            }
        }
    }

    (drawn_numbers, boards)
}
