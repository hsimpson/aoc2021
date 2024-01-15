use crate::utils;
use std::collections::HashMap;
use std::time::Instant;

type Rules = HashMap<String, char>;
type Occurrences = HashMap<char, u64>;

fn create_rules(input: &String) -> Rules {
    let mut rules: Rules = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    for line in 2..lines.len() {
        let rule: Vec<&str> = lines[line].split(" -> ").collect();
        rules.insert(rule[0].to_string(), rule[1].chars().next().unwrap());
    }
    return rules;
}

fn step(template: &String, rules: &Rules) -> String {
    let mut new_template = String::new();
    let chars = template.chars().collect::<Vec<char>>();
    for i in 0..chars.len() - 1 {
        let pair = chars[i..i + 2].iter().collect::<String>();
        let insert = rules.get(&pair).unwrap();

        if i == 0 {
            new_template.push(chars[i]);
        }
        new_template.push(*insert);
        new_template.push(chars[i + 1]);
    }

    return new_template;
}

fn steps(occurences: &mut Occurrences, rules: &Rules, steps: u32) {}

fn get_occurrences(template: &String) -> Occurrences {
    let mut occurrences: Occurrences = HashMap::new();
    for c in template.chars() {
        let count = occurrences.entry(c).or_insert(0);
        *count += 1;
    }
    return occurrences;
}

pub fn puzzle1() {
    println!("Day 14, puzzle 1");
    let start = Instant::now();
    let input = utils::file::read_input("src/day14/input.txt");

    let rules = create_rules(&input);
    let mut template = input.lines().nth(0).unwrap().to_string();

    for _ in 0..10 {
        template = step(&template, &rules);
        // println!("{}", template);
    }

    let occurrences = get_occurrences(&template);
    let most_common = occurrences.iter().max_by_key(|&(_, v)| v).unwrap();
    let least_common = occurrences.iter().min_by_key(|&(_, v)| v).unwrap();

    let result = most_common.1 - least_common.1;
    println!("Result: {}", result);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    println!("Day 14, puzzle 2");
    let start = Instant::now();
    let input = utils::file::read_input("src/day14/test.txt");

    let rules = create_rules(&input);
    let mut template = input.lines().nth(0).unwrap().to_string();
    let mut occurrences = get_occurrences(&template);

    steps(&mut occurrences, &rules, 10);

    let most_common = occurrences.iter().max_by_key(|&(_, v)| v).unwrap();
    let least_common = occurrences.iter().min_by_key(|&(_, v)| v).unwrap();

    let result = most_common.1 - least_common.1;
    println!("Result: {}", result);
    println!("Time elapsed: {:?}", start.elapsed());
}
