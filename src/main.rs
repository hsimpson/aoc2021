use std::time::Instant;
mod utils {
    pub mod file;
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;

fn main() {
    println!("");
    println!("Advent of Code 2021");
    let start = Instant::now();
    day01::puzzle1();
    day01::puzzle2();

    println!("");
    day02::puzzle1();
    day02::puzzle2();

    println!("");
    day03::puzzle1();
    day03::puzzle2();

    println!("");
    day04::puzzle1();
    day04::puzzle2();

    println!("");
    day05::puzzle1();
    day05::puzzle2();

    println!("");
    day06::puzzle1();
    day06::puzzle2();

    println!("");
    day07::puzzle1();
    day07::puzzle2();

    println!("");
    day08::puzzle1();
    day08::puzzle2();

    println!("");
    day09::puzzle1();
    day09::puzzle2();

    println!("");
    day10::puzzle1();
    day10::puzzle2();

    println!("");
    day11::puzzle1();
    day11::puzzle2();

    println!("");
    day12::puzzle1();
    day12::puzzle2();

    // println!("");
    // day13::puzzle1();
    // day13::puzzle2();

    // println!("");
    // day14::puzzle1();
    // // day14::puzzle2();

    // println!("");
    // day15::puzzle1();
    // //day15::puzzle2();

    // println!("");
    // day16::puzzle1();
    // day16::puzzle2();

    // println!("");
    // day17::puzzle1();
    // day17::puzzle2();

    // println!("");
    // day18::puzzle1();
    // day18::puzzle2();

    println!("");
    println!("Time elapsed overall: {:?}", start.elapsed());
}
