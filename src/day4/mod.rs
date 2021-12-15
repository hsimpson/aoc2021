use std::time::Instant;

use crate::utils;

struct Board {
    numbers: [[u8; 5]; 5],
    hit: [[bool; 5]; 5],
    has_bingo: bool,
}

fn create_board(numbers: &Vec<u8>) -> Board {
    let mut board = Board {
        numbers: [[0; 5]; 5],
        hit: [[false; 5]; 5],
        has_bingo: false,
    };

    for (i, number) in numbers.iter().enumerate() {
        let row = i / 5;
        let col = i % 5;
        board.numbers[row][col] = *number;
    }

    board
}

fn create_boards(lines: Vec<&str>) -> Vec<Board> {
    // create the boards
    let mut boards: Vec<Board> = Vec::new();
    let mut row = 0;
    let mut board_numbers: Vec<u8> = Vec::new();
    for line in lines.iter().skip(1) {
        if *line == "" {
            continue;
        }
        row += 1;

        let mut line_numbers = line
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        board_numbers.append(&mut line_numbers);

        if row % 5 == 0 {
            let board = create_board(&board_numbers);
            boards.push(board);
            board_numbers.clear();
        }
    }
    return boards;
}

fn has_hit(board: &mut Board, number: u8) -> bool {
    let mut hit = false;
    for row in 0..5 {
        for col in 0..5 {
            if board.numbers[row][col] == number {
                hit = true;
                board.hit[row][col] = true;
            }
        }
    }
    return hit;
}

fn has_bingo(board: &Board) -> bool {
    // test rows
    for row in 0..5 {
        let mut bingo = true;
        for col in 0..5 {
            if !board.hit[row][col] {
                bingo = false;
            }
        }
        if bingo {
            return true;
        }
    }

    // test columns
    for col in 0..5 {
        let mut bingo = true;
        for row in 0..5 {
            if !board.hit[row][col] {
                bingo = false;
            }
        }
        if bingo {
            return true;
        }
    }

    return false;
}

fn calc_sum(board: &Board) -> u32 {
    let mut sum = 0;
    for row in 0..5 {
        for col in 0..5 {
            if !board.hit[row][col] {
                sum += board.numbers[row][col] as u32;
            }
        }
    }

    return sum;
}

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 4, puzzle 1");

    let input = utils::file::read_input("src/day4/input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let test_numbers = lines[0]
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let mut boards: Vec<Board> = Vec::new();

    // create the boards
    let mut row = 0;
    let mut board_numbers: Vec<u8> = Vec::new();
    for line in lines.iter().skip(1) {
        if *line == "" {
            continue;
        }
        row += 1;

        let mut line_numbers = line
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        board_numbers.append(&mut line_numbers);

        if row % 5 == 0 {
            let board = create_board(&board_numbers);
            boards.push(board);
            board_numbers.clear();
        }
    }

    // start bingo game and test numbers
    for number in test_numbers.iter() {
        for board in boards.iter_mut() {
            let hit = has_hit(board, *number);

            if hit {
                if has_bingo(board) {
                    println!("Bingo, score: {}", calc_sum(board) * (*number as u32));
                    println!("Time elapsed: {:?}", start.elapsed());
                    return;
                }
            }
        }
    }

    println!("No Bingo! This should never happen!");
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 4, puzzle 2");

    let input = utils::file::read_input("src/day4/input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let test_numbers = lines[0]
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let mut boards = create_boards(lines);

    // start bingo game and test numbers
    let mut bingo_complete = 0;
    for number in test_numbers.iter() {
        let board_length = boards.len();
        for board in boards.iter_mut() {
            if board.has_bingo {
                continue;
            }
            let hit = has_hit(board, *number);

            if hit {
                if has_bingo(board) {
                    board.has_bingo = true;
                    bingo_complete += 1;
                    if bingo_complete == board_length {
                        println!("Bingo, score: {}", calc_sum(board) * (*number as u32));
                        println!("Time elapsed: {:?}", start.elapsed());
                        return;
                    }
                }
            }
        }
    }

    println!("Time elapsed: {:?}", start.elapsed());
}
