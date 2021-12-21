use std::time::Instant;

use crate::utils;

pub fn puzzle1() {
    println!("Day 7, puzzle 1");
    let start = Instant::now();
    let input = utils::file::read_input("src/day07/input.txt");
    let mut horizontal_postions: Vec<u32> = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    horizontal_postions.sort();

    let best_position = horizontal_postions[horizontal_postions.len() / 2];
    let mut fuel = 0;

    for pos in horizontal_postions {
        if pos < best_position {
            fuel += best_position - pos;
        } else if pos > best_position {
            fuel += pos - best_position;
        }
    }

    println!("Fuel required: {}", fuel);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    println!("Day 7, puzzle 2");
    let start = Instant::now();
    let input = utils::file::read_input("src/day07/input.txt");
    let horizontal_postions: Vec<u32> = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    // horizontal_postions.sort();

    let sum = horizontal_postions.iter().sum::<u32>();
    let avg = sum as f32 / horizontal_postions.len() as f32;
    let best_position = avg.floor() as u32; // don't know why this needs to be floored
    let mut fuel = 0;

    for pos in horizontal_postions {
        let mut steps = 0;
        if pos < best_position {
            steps = best_position - pos;
        } else if pos > best_position {
            steps = pos - best_position;
        }

        fuel += steps * (steps + 1) / 2;
    }

    println!("Fuel required: {}", fuel);
    println!("Time elapsed: {:?}", start.elapsed());
}
