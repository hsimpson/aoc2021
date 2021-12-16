use std::cmp;
use std::time::Instant;

use crate::utils;

struct Point {
    x: u32,
    y: u32,
}

struct Line {
    start: Point,
    end: Point,
}

fn create_lines(str_lines: Vec<&str>, max_x: &mut u32, max_y: &mut u32) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();

    for line in str_lines.iter() {
        let start_end = line.split(" -> ").collect::<Vec<&str>>();

        let start = start_end[0].split(",").collect::<Vec<&str>>();
        let end = start_end[1].split(",").collect::<Vec<&str>>();

        let x1 = start[0].parse::<u32>().unwrap();
        let y1 = start[1].parse::<u32>().unwrap();
        let x2 = end[0].parse::<u32>().unwrap();
        let y2 = end[1].parse::<u32>().unwrap();

        *max_x = cmp::max(*max_x, x1);
        *max_x = cmp::max(*max_x, x2);
        *max_y = cmp::max(*max_y, y1);
        *max_y = cmp::max(*max_y, y2);

        let line = Line {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        };

        lines.push(line);
    }

    return lines;
}

fn is_line_diagonal(line: &Line) -> bool {
    let horizontal = line.start.y == line.end.y;
    let vertical = line.start.x == line.end.x;
    return !horizontal && !vertical;
}

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 5, puzzle 1");

    let input = utils::file::read_input("src/day5/test.txt");
    let str_lines: Vec<&str> = input.lines().collect();

    let mut max_x = 0;
    let mut max_y = 0;
    let lines = create_lines(str_lines, &mut max_x, &mut max_y);

    let mut grid: Vec<Vec<u32>> = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    let mut covered_points = 0;

    for line in lines.iter() {
        if is_line_diagonal(line) {
            continue;
        }

        let min_x1 = cmp::min(line.start.x, line.end.x);
        let max_x1 = cmp::max(line.start.x, line.end.x);
        let min_y1 = cmp::min(line.start.y, line.end.y);
        let max_y1 = cmp::max(line.start.y, line.end.y);

        for x in min_x1..=max_x1 {
            for y in min_y1..=max_y1 {
                grid[y as usize][x as usize] += 1;
                if grid[y as usize][x as usize] == 2 {
                    covered_points += 1;
                }
            }
        }
    }
    println!("Covered points: {}", covered_points);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 5, puzzle 2");

    let input = utils::file::read_input("src/day5/input.txt");
    let str_lines: Vec<&str> = input.lines().collect();

    let mut max_x = 0;
    let mut max_y = 0;
    let lines = create_lines(str_lines, &mut max_x, &mut max_y);

    let mut grid: Vec<Vec<u32>> = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    let mut covered_points = 0;

    for line in lines.iter() {
        if is_line_diagonal(line) {
            let mut x = line.start.x;
            let mut y = line.start.y;

            loop {
                grid[y as usize][x as usize] += 1;
                if grid[y as usize][x as usize] == 2 {
                    covered_points += 1;
                }

                if x == line.end.x || y == line.end.y {
                    break;
                }

                if line.start.x < line.end.x {
                    x += 1
                } else {
                    x -= 1
                }
                if line.start.y < line.end.y {
                    y += 1
                } else {
                    y -= 1
                }
            }
        } else {
            let min_x1 = cmp::min(line.start.x, line.end.x);
            let max_x1 = cmp::max(line.start.x, line.end.x);
            let min_y1 = cmp::min(line.start.y, line.end.y);
            let max_y1 = cmp::max(line.start.y, line.end.y);
            for x in min_x1..=max_x1 {
                for y in min_y1..=max_y1 {
                    grid[y as usize][x as usize] += 1;
                    if grid[y as usize][x as usize] == 2 {
                        covered_points += 1;
                    }
                }
            }
        }
    }
    println!("Covered points: {}", covered_points);
    println!("Time elapsed: {:?}", start.elapsed());
}
