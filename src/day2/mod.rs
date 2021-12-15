use crate::utils;
use std::time::Instant;

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 2, puzzle 1");

    let input = utils::file::read_input("src/day2/input.txt");
    let commands = input.lines().collect::<Vec<&str>>();

    let mut depth = 0;
    let mut horizontal = 0;

    for n in 0..commands.len() {
        let command: Vec<&str> = commands[n].split(" ").collect();
        let direction = command[0];
        let distance = command[1].parse::<i32>().unwrap();

        match direction {
            "forward" => horizontal += distance,
            "down" => depth += distance,
            "up" => depth -= distance,
            _ => panic!("Unknown direction"),
        }
    }

    println!("horizontal * depth = {}", horizontal * depth);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 2, puzzle 2");

    let input = utils::file::read_input("src/day2/input.txt");
    let commands = input.lines().collect::<Vec<&str>>();

    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for n in 0..commands.len() {
        let command: Vec<&str> = commands[n].split(" ").collect();
        let direction = command[0];
        let distance = command[1].parse::<i32>().unwrap();

        match direction {
            "forward" => {
                horizontal += distance;
                depth += aim * distance;
            }
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => panic!("Unknown direction"),
        }
    }

    println!("horizontal * depth = {}", horizontal * depth);
    println!("Time elapsed: {:?}", start.elapsed());
}
