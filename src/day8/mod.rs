use crate::utils;
use std::collections::HashMap;
use std::time::Instant;

pub fn puzzle1() {
    println!("Day 8, puzzle 1");
    let start = Instant::now();
    let input = utils::file::read_input("src/day8/input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut unique_digits = 0;

    for line in lines {
        let input_parts: Vec<&str> = line.split(" | ").collect();
        let right_parts: Vec<&str> = input_parts[1].split(" ").collect();

        for right_part in right_parts {
            let len = right_part.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                unique_digits += 1;
            }
        }
    }
    println!("Unique digits: {}", unique_digits);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn contains_digit(digit: &String, to_contain: &String) -> bool {
    let digit_chars: Vec<char> = digit.chars().collect();
    let to_contain_chars: Vec<char> = to_contain.chars().collect();

    for c in to_contain_chars {
        if !digit_chars.contains(&c) {
            return false;
        }
    }
    return true;
}

pub fn sort_chars(input: &String) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();
    return chars.into_iter().collect();
}

pub fn get_digits(unknown: &Vec<String>) -> HashMap<String, char> {
    let mut digits: HashMap<String, char> = HashMap::new();

    // first set the safe digits (1, 7, 4, 8)
    digits.insert(sort_chars(&unknown[0]).clone(), '1'); // digit 1
    digits.insert(sort_chars(&unknown[1]).clone(), '7'); // digit 7
    digits.insert(sort_chars(&unknown[2]).clone(), '4'); // digit 4
    digits.insert(sort_chars(&unknown[9]).clone(), '8'); // digit 8

    // safe the digit 6 for later use
    let mut digit_6: String = String::new();

    // check the digits with length 6 (0, 6, 9)
    for i in 6..=8 {
        // if contains the digit 4, then it must be a 9
        // else if contains digit 1, then it must be a 0
        // else it must be a 6
        if contains_digit(&unknown[i], &unknown[2]) {
            digits.insert(sort_chars(&unknown[i]).clone(), '9');
        } else if contains_digit(&unknown[i], &unknown[0]) {
            digits.insert(sort_chars(&unknown[i]).clone(), '0');
        } else {
            digits.insert(sort_chars(&unknown[i]).clone(), '6');
            digit_6 = unknown[i].clone();
        }
    }

    // check the digits with length 5 (2, 3, 5)
    for i in 3..=5 {
        // if contains the digit 7, then it must be a 3
        // else if a 6 containes this, then it must be a 5
        // else it must be a 2
        if contains_digit(&unknown[i], &unknown[1]) {
            digits.insert(sort_chars(&unknown[i]).clone(), '3');
        } else if contains_digit(&digit_6, &unknown[i]) {
            digits.insert(sort_chars(&unknown[i]).clone(), '5');
        } else {
            digits.insert(sort_chars(&unknown[i]).clone(), '2');
        }
    }

    return digits;
}

pub fn puzzle2() {
    println!("Day 8, puzzle 2");
    let start = Instant::now();
    let input = utils::file::read_input("src/day8/input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for line in lines {
        let input_parts: Vec<&str> = line.split(" | ").collect();

        // split the line
        let mut left_parts: Vec<String> =
            input_parts[0].split(" ").map(|s| s.to_string()).collect();
        let right_parts: Vec<String> = input_parts[1].split(" ").map(|s| s.to_string()).collect();
        // sort by length
        left_parts.sort_by(|a, b| a.len().cmp(&b.len()));

        // get the digits
        let digits = get_digits(&left_parts);
        let mut right_digits = String::new();

        for right_part in right_parts {
            // sort the chars in right_part
            let right_part_sorted = sort_chars(&right_part);
            let digit = digits.get(&right_part_sorted).unwrap();
            right_digits.push(*digit);
        }

        let number = right_digits.parse::<u32>().unwrap();
        sum += number;
    }

    println!("Sum: {}", sum);
    println!("Time elapsed: {:?}", start.elapsed());
}
