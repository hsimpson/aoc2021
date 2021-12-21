use crate::utils;
use std::time::Instant;

struct Grid {
    octopuses: Vec<Vec<u32>>,
    width: usize,
    height: usize,
    flashes: Vec<(usize, usize)>,
    flash_count: u32,
}

fn get_adjacent(grid: &Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut adjacents: Vec<(usize, usize)> = Vec::new();

    if y > 0 {
        if x > 0 {
            adjacents.push((x - 1, y - 1)); // top left
        }
        adjacents.push((x, y - 1)); // top
        if x < grid.width - 1 {
            adjacents.push((x + 1, y - 1)); // top right
        }
    }
    if x > 0 {
        adjacents.push((x - 1, y)); // left
    }

    if y < grid.height - 1 {
        if x > 0 {
            adjacents.push((x - 1, y + 1)); // bottom left
        }
        adjacents.push((x, y + 1)); // bottom
        if x < grid.width - 1 {
            adjacents.push((x + 1, y + 1)); // bottom right
        }
    }
    if x < grid.width - 1 {
        adjacents.push((x + 1, y)); // right
    }

    return adjacents;
}

fn create_grid(input: &String) -> Grid {
    let lines: Vec<&str> = input.lines().collect();

    let mut grid = Grid {
        octopuses: Vec::new(),
        width: lines[0].len(),
        height: lines.len(),
        flashes: Vec::new(),
        flash_count: 0,
    };

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let line_numbers: Vec<u32> = chars.iter().map(|c| c.to_digit(10).unwrap()).collect();
        grid.octopuses.push(line_numbers);
    }

    return grid;
}

fn has_flashed(grid: &Grid, x: usize, y: usize) -> bool {
    for (x_flashed, y_flashed) in grid.flashes.iter() {
        if x == *x_flashed && y == *y_flashed {
            return true;
        }
    }
    return false;
}

fn increase(grid: &mut Grid, x: usize, y: usize) {
    if has_flashed(grid, x, y) {
        return;
    }

    let mut value = grid.octopuses[y][x];
    value += 1;

    if value > 9 {
        value = 0;
        grid.flashes.push((x, y));
        grid.octopuses[y][x] = value;
        grid.flash_count += 1;

        let adjacents = get_adjacent(grid, x, y);
        for adjacent in adjacents {
            increase(grid, adjacent.0, adjacent.1);
        }
    }
    grid.octopuses[y][x] = value;
}

fn step(grid: &mut Grid) {
    grid.flashes.clear();
    for y in 0..grid.height {
        for x in 0..grid.width {
            increase(grid, x, y);
        }
    }
}

pub fn puzzle1() {
    println!("Day 11, puzzle 1");
    let start = Instant::now();
    let input = utils::file::read_input("src/day11/input.txt");

    let mut grid = create_grid(&input);

    for _ in 0..100 {
        step(&mut grid);
    }

    println!("Flashes: {}", grid.flash_count);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    println!("Day 11, puzzle 2");
    let start = Instant::now();
    let input = utils::file::read_input("src/day11/input.txt");

    let mut grid = create_grid(&input);

    let octupus_count = grid.width * grid.height;
    let mut steps = 1;
    loop {
        step(&mut grid);
        if grid.flashes.len() == octupus_count {
            break;
        }
        steps += 1;
    }

    println!("Step : {}", steps);
    println!("Time elapsed: {:?}", start.elapsed());
}
