use crate::utils;
use std::time::Instant;

const ROUND_BRACKET_OPEN: char = '(';
const ROUND_BRACKET_CLOSE: char = ')';
const SQUARE_BRACKET_OPEN: char = '[';
const SQUARE_BRACKET_CLOSE: char = ']';
const CURLY_BRACKET_OPEN: char = '{';
const CURLY_BRACKET_CLOSE: char = '}';
const ANGLE_BRACKET_OPEN: char = '<';
const ANGLE_BRACKET_CLOSE: char = '>';

enum ParserStatus {
    Incomplete,
    Corrupted,
}

struct ParseInfo {
    pos: usize,
    start_tokens: Vec<char>,
    end_tokens: Vec<char>,
    status: ParserStatus,
    corrupted_token: char,
}

fn is_open_token(token: char) -> bool {
    return token == ROUND_BRACKET_OPEN
        || token == SQUARE_BRACKET_OPEN
        || token == CURLY_BRACKET_OPEN
        || token == ANGLE_BRACKET_OPEN;
}

fn parse_token(tok: char, pos: usize, parse_info: &mut ParseInfo) {
    parse_info.pos = pos;
    if is_open_token(tok) {
        parse_info.start_tokens.push(tok);
        if tok == ROUND_BRACKET_OPEN {
            parse_info.end_tokens.push(ROUND_BRACKET_CLOSE);
        } else if tok == SQUARE_BRACKET_OPEN {
            parse_info.end_tokens.push(SQUARE_BRACKET_CLOSE);
        } else if tok == CURLY_BRACKET_OPEN {
            parse_info.end_tokens.push(CURLY_BRACKET_CLOSE);
        } else if tok == ANGLE_BRACKET_OPEN {
            parse_info.end_tokens.push(ANGLE_BRACKET_CLOSE);
        }
    } else {
        let close_token = parse_info.end_tokens.last().unwrap();
        if tok == *close_token {
            parse_info.start_tokens.pop();
            parse_info.end_tokens.pop();
        } else {
            parse_info.status = ParserStatus::Corrupted;
            parse_info.corrupted_token = tok;
        }
    }
}

fn parse(line: &String) -> ParseInfo {
    let mut info = ParseInfo {
        pos: 0,
        start_tokens: vec![],
        end_tokens: vec![],
        status: ParserStatus::Incomplete,
        corrupted_token: '\0',
    };

    for (i, c) in line.chars().enumerate() {
        parse_token(c, i + 1, &mut info);
        match info.status {
            ParserStatus::Incomplete => {}
            ParserStatus::Corrupted => {
                return info;
            }
        }
    }

    return info;
}

pub fn puzzle1() {
    println!("Day 10, puzzle 1");
    let start = Instant::now();
    let input = utils::file::read_input("src/day10/input.txt");
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let mut score = 0;
    for line in lines {
        let parser_status = parse(&line);

        match parser_status.corrupted_token {
            ROUND_BRACKET_CLOSE => {
                score += 3;
            }
            SQUARE_BRACKET_CLOSE => {
                score += 57;
            }
            CURLY_BRACKET_CLOSE => {
                score += 1197;
            }
            ANGLE_BRACKET_CLOSE => {
                score += 25137;
            }
            _ => {}
        }
    }
    println!("Score: {}", score);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    println!("Day 10, puzzle 2");
    let start = Instant::now();
    let input = utils::file::read_input("src/day10/input.txt");
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let mut scores: Vec<u64> = vec![];
    for line in lines {
        let parser_status = parse(&line);
        match parser_status.status {
            ParserStatus::Corrupted => {}
            ParserStatus::Incomplete => {
                let mut score: u64 = 0;
                for close in parser_status.end_tokens.iter().rev() {
                    score *= 5;
                    match *close {
                        ROUND_BRACKET_CLOSE => {
                            score += 1;
                        }
                        SQUARE_BRACKET_CLOSE => {
                            score += 2;
                        }
                        CURLY_BRACKET_CLOSE => {
                            score += 3;
                        }
                        ANGLE_BRACKET_CLOSE => {
                            score += 4;
                        }
                        _ => {}
                    }
                }

                scores.push(score);
            }
        }
    }
    scores.sort();
    let middle: usize = scores.len() / 2;

    println!("Score: {}", scores[middle]);
    println!("Time elapsed: {:?}", start.elapsed());
}
