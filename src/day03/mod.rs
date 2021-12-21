use std::collections::LinkedList;
use std::time::Instant;

use crate::utils;

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 3, puzzle 1");

    let input = utils::file::read_input("src/day03/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let bit_length: usize = lines[0].len();

    let mut array: Vec<u32> = vec![0; bit_length];
    let length = lines.len();

    for i in 0..length {
        let line = lines[i];
        for j in 0..bit_length {
            let ch = line.chars().nth(j).unwrap();
            if ch == '1' {
                array[j] += 1;
            }
        }
    }

    let half = length as u32 / 2;

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for i in 0..bit_length {
        // println!("{}", array[i]);
        let num = array[i];
        if num > half {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
        if i < bit_length - 1 {
            gamma <<= 1;
            epsilon <<= 1;
        }
    }

    // println!("{:?}", gamma);
    // println!("{:?}", epsilon);
    println!("gamma * epsilon = {}", gamma * epsilon);
    println!("Time elapsed: {:?}", start.elapsed());
}

fn puzzle2_calc_low_height(shift: usize, list: &LinkedList<u32>, more: bool) -> u32 {
    let mut set: LinkedList<u32> = LinkedList::new();
    let mut unset: LinkedList<u32> = LinkedList::new();
    for element in list.iter() {
        if element & (1 << shift) != 0 {
            set.push_back(*element);
        } else {
            unset.push_back(*element);
        }
    }

    let mut new_list: LinkedList<u32>;
    let len_set = set.len();
    let len_unset = unset.len();

    if more {
        new_list = if len_set > len_unset || len_set == len_unset {
            set
        } else {
            unset
        };
    } else {
        new_list = if len_set > len_unset || len_set == len_unset {
            unset
        } else {
            set
        };
    }

    if new_list.len() == 1 {
        return new_list.pop_front().unwrap();
    } else {
        return puzzle2_calc_low_height(shift - 1, &new_list, more);
    }
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 3, puzzle 2");

    let input = utils::file::read_input("src/day03/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let bit_length: usize = lines[0].len();

    let mut number_list: LinkedList<u32> = LinkedList::new();

    lines.iter().for_each(|line| {
        let num = u32::from_str_radix(line, 2).unwrap();
        number_list.push_back(num);
    });

    let o2_generator_rating = puzzle2_calc_low_height(bit_length - 1, &number_list, true);
    let co2_scrubber_rating = puzzle2_calc_low_height(bit_length - 1, &number_list, false);

    // println!("oxygen generator rating: {}", o2_generator_rating);
    // println!("CO2 scrubber rating: {}", co2_scrubber_rating);

    println!(
        "oxygen generator rating * CO2 scrubber rating: {}",
        o2_generator_rating * co2_scrubber_rating
    );
    println!("Time elapsed: {:?}", start.elapsed());
}
