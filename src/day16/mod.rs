use crate::utils;
use std::collections::HashMap;
use std::time::Instant;

fn convert_to_binary(input: &String) -> String {
    let mut binaries: HashMap<char, &str> = HashMap::new();
    binaries.insert('0', "0000");
    binaries.insert('1', "0001");
    binaries.insert('2', "0010");
    binaries.insert('3', "0011");
    binaries.insert('4', "0100");
    binaries.insert('5', "0101");
    binaries.insert('6', "0110");
    binaries.insert('7', "0111");
    binaries.insert('8', "1000");
    binaries.insert('9', "1001");
    binaries.insert('A', "1010");
    binaries.insert('B', "1011");
    binaries.insert('C', "1100");
    binaries.insert('D', "1101");
    binaries.insert('E', "1110");
    binaries.insert('F', "1111");
    let mut result = String::new();
    for c in input.chars() {
        result.push_str(binaries.get(&c).unwrap());
    }
    return result;
}

#[derive(Debug)]
struct Packet {
    version: u32,
    id: u32,
    literal: Option<u64>,
    subpackes: Vec<Packet>,
}

fn parse_version(input: &String, packet: &mut Packet, pos: &mut usize) {
    let version_str = &input[*pos..*pos + 3];
    let version = isize::from_str_radix(version_str, 2).unwrap();
    packet.version = version as u32;

    *pos = *pos + 3;
}

fn parse_id(input: &String, packet: &mut Packet, pos: &mut usize) {
    let id_str = &input[*pos..*pos + 3];
    let id = isize::from_str_radix(id_str, 2).unwrap();
    packet.id = id as u32;

    *pos = *pos + 3;
}

fn parse_literal(input: &String, packet: &mut Packet, pos: &mut usize) {
    let chars: Vec<char> = input.chars().collect();
    let mut literal_str = String::new();
    loop {
        let last = chars[*pos] == '0';
        literal_str.push_str(&input[*pos + 1..*pos + 5]);
        *pos = *pos + 5;
        if last {
            // *pos = *pos + (*pos + 1) % 4;
            break;
        }
    }
    let literal = isize::from_str_radix(&literal_str[..], 2).unwrap();
    packet.literal = Some(literal as u64);
}

fn parse_operator(input: &String, packet: &mut Packet, pos: &mut usize) {
    let chars: Vec<char> = input.chars().collect();
    let length_id = chars[*pos];
    if length_id == '0' {
        // the next 15 bits are the total length of bits of the subpackets
        let length_str = &input[*pos + 1..*pos + 16];
        let length = isize::from_str_radix(length_str, 2).unwrap();
        *pos = *pos + 16;
        let length_offset = *pos + length as usize;
        while *pos < length_offset {
            packet.subpackes.push(parse_packet(input, pos));
        }
    } else {
        let count_str = &input[*pos + 1..*pos + 12];
        let count = isize::from_str_radix(count_str, 2).unwrap();
        *pos = *pos + 12;

        for _ in 0..count {
            packet.subpackes.push(parse_packet(input, pos));
        }
    }
}

fn parse_packet(input: &String, pos: &mut usize) -> Packet {
    let mut packet = Packet {
        version: 0,
        id: 0,
        literal: None,
        subpackes: Vec::new(),
    };

    parse_version(&input, &mut packet, pos);
    parse_id(&input, &mut packet, pos);
    if packet.id == 4 {
        parse_literal(&input, &mut packet, pos);
    } else {
        parse_operator(&input, &mut packet, pos);
    }
    return packet;
}

fn add_version_numbers(packet: Packet) -> u32 {
    let mut result = packet.version;
    for subpacket in packet.subpackes {
        result = result + add_version_numbers(subpacket);
    }
    return result;
}

fn sum(packets: &Vec<Packet>) -> u64 {
    let mut result = 0;
    for (i, packet) in packets.iter().enumerate() {
        result = result + get_number(packet);
        if i < packets.len() - 1 {
            print!(" + ");
        }
    }
    return result;
}

fn mul(packets: &Vec<Packet>) -> u64 {
    let mut result = 1;
    for (i, packet) in packets.iter().enumerate() {
        result = result * get_number(&packet);
        if i < packets.len() - 1 {
            print!(" * ");
        }
    }
    return result;
}

fn min(packets: &Vec<Packet>) -> u64 {
    let mut result = std::u64::MAX;
    print!("min(");
    for (i, packet) in packets.iter().enumerate() {
        result = std::cmp::min(result, get_number(&packet));
        if i < packets.len() - 1 {
            print!(", ");
        }
    }
    print!(")");
    return result;
}

fn max(packets: &Vec<Packet>) -> u64 {
    let mut result = 0;
    print!("max(");
    for (i, packet) in packets.iter().enumerate() {
        result = std::cmp::max(result, get_number(&packet));
        if i < packets.len() - 1 {
            print!(", ");
        }
    }
    print!(")");
    return result;
}

fn greater(packets: &Vec<Packet>) -> u64 {
    let a = get_number(&packets[0]);
    print!(" > ");
    let b = get_number(&packets[1]);
    if a > b {
        return 1;
    }
    return 0;
}

fn less(packets: &Vec<Packet>) -> u64 {
    let a = get_number(&packets[0]);
    print!(" < ");
    let b = get_number(&packets[1]);
    if a < b {
        return 1;
    }
    return 0;
}

fn equal(packets: &Vec<Packet>) -> u64 {
    let a = get_number(&packets[0]);
    print!(" == ");
    let b = get_number(&packets[1]);
    if a == b {
        return 1;
    }
    return 0;
}

fn get_number(packet: &Packet) -> u64 {
    if packet.literal.is_some() {
        print!("{}", packet.literal.unwrap());
        return packet.literal.unwrap();
    }
    return solve_operator(packet);
}

fn solve_operator(packet: &Packet) -> u64 {
    let mut result = 0;
    match packet.id {
        0 => {
            result = sum(&packet.subpackes);
        }
        1 => {
            result = mul(&packet.subpackes);
        }
        2 => {
            result = min(&packet.subpackes);
        }
        3 => {
            result = max(&packet.subpackes);
        }
        5 => {
            result = greater(&packet.subpackes);
        }
        6 => {
            result = less(&packet.subpackes);
        }
        7 => {
            result = equal(&packet.subpackes);
        }
        _ => println!("Unknown operator: {}", packet.id),
    }
    return result;
}

pub fn puzzle1() {
    println!("Day 16, puzzle 1");
    let start = Instant::now();
    let input = utils::file::read_input("src/day16/input.txt");
    let binaries = convert_to_binary(&input);
    let transmission = parse_packet(&binaries, &mut 0);

    let sum_version_numbers = add_version_numbers(transmission);

    println!("The sum of the version numbers is {}", sum_version_numbers);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    println!("Day 16, puzzle 2");
    let start = Instant::now();
    let input = utils::file::read_input("src/day16/input.txt");
    let binaries = convert_to_binary(&input);
    let transmission = parse_packet(&binaries, &mut 0);
    println!("Equation:");
    let result = solve_operator(&transmission);
    println!();

    println!("The result is {}", result);
    println!("Time elapsed: {:?}", start.elapsed());
}
