use crate::utils;

pub fn puzzle1() {
    println!("Day 1, puzzle 1");

    let input = utils::file::read_input("src/day1/input.txt");
    let string_array: Vec<&str> = input.split("\n").collect();

    let mut increases = 0;

    for n in 1..string_array.len() {

        let current = string_array[n].parse::<i32>().unwrap();
        let prev = string_array[n-1].parse::<i32>().unwrap();
        
        if current > prev {
            increases += 1;
        }
    }

    println!("increases: {}", increases);
}

pub fn puzzle2() {
    println!("Day 1, puzzle 2");

    let input = utils::file::read_input("src/day1/input.txt");
    let string_array: Vec<&str> = input.split("\n").collect();

    let mut increases = 0;
    let mut sum = 0;

    for n in 0..string_array.len()-2 {
        let first = string_array[n].parse::<i32>().unwrap();
        let second = string_array[n+1].parse::<i32>().unwrap();
        let third = string_array[n+2].parse::<i32>().unwrap();

        
        let current_sum = first + second + third;
        if n > 0 {
            if current_sum > sum {
                increases += 1;
            }
        }
        sum = current_sum;
    }

    println!("increases: {}", increases);

}