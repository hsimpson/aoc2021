use std::time::Instant;

use crate::utils;

fn simulate(lanternfishes: &mut Vec<u64>) {
    // check zeros
    let zeros = lanternfishes[0];
    let mut prev: u64 = 0;
    for i in (0..9).rev() {
        let current = lanternfishes[i];
        lanternfishes[i] = prev;
        prev = current;
    }

    if zeros > 0 {
        lanternfishes[8] += zeros;
        lanternfishes[6] += zeros;
    }
}

fn run(days: u32) {
    let start = Instant::now();
    let input = utils::file::read_input("src/day06/input.txt");
    let lanternfishes_initial: Vec<u8> =
        input.split(',').map(|x| x.parse::<u8>().unwrap()).collect();

    let mut lanternfishes: Vec<u64> = vec![0; 9];

    for lifetime in lanternfishes_initial.iter() {
        lanternfishes[*lifetime as usize] += 1;
    }

    for _ in 0..days {
        simulate(&mut lanternfishes);
    }

    // acumulate the lanternfishes
    let mut sum = 0;
    for lanternfish in lanternfishes.iter() {
        sum += *lanternfish;
    }

    println!("Lanternfishes: {}", sum);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle1() {
    println!("Day 6, puzzle 1");
    run(80);
}

pub fn puzzle2() {
    println!("Day 6, puzzle 2");
    run(256);
}
