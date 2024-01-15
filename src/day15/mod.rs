use crate::utils;
use pathfinding::prelude::astar;
use std::time::Instant;

struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<u32>>,
}

fn create_grid(input: &String) -> Grid {
    let lines: Vec<&str> = input.lines().collect();
    let mut grid = Grid {
        width: lines[0].len(),
        height: lines.len(),
        data: vec![vec![0; input.len()]; input.lines().count()],
    };

    for row in 0..grid.height {
        let chars = lines[row].chars().collect::<Vec<char>>();

        for col in 0..grid.width {
            grid.data[row][col] = chars[col].to_digit(10).unwrap();
        }
    }

    return grid;
}

fn create_large_grid(input: &String) -> Grid {
    let sub_grid = create_grid(input);
    let large_grid_width = sub_grid.width * 5;
    let large_grid_height = sub_grid.height * 5;
    let mut grid = Grid {
        width: large_grid_width,
        height: large_grid_height,
        data: vec![vec![0; large_grid_width]; large_grid_height],
    };

    let mut add = 0;
    let mut add_row = 0;
    for row in 0..large_grid_height {
        for col in 0..large_grid_width {
            let mut value = sub_grid.data[row % sub_grid.width][col % sub_grid.height] + add;
            if value > 9 {
                value -= 9;
            }
            grid.data[row][col] = value;

            if (col + 1) % sub_grid.width == 0 {
                add += 1;
            }
        }
        if (row + 1) % sub_grid.height == 0 {
            add_row += 1;
        }
        add = add_row;
    }

    return grid;
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

fn successors(pos: &Pos, grid: &Grid) -> Vec<(Pos, u32)> {
    let mut successors: Vec<(Pos, u32)> = Vec::new();

    let x = pos.0;
    let y = pos.1;

    if x < grid.width - 1 {
        successors.push((Pos(x + 1, y), grid.data[y][x + 1]));
    }
    if y < grid.height - 1 {
        successors.push((Pos(x, y + 1), grid.data[y + 1][x]));
    }

    return successors;
}

fn distance(pos: &Pos, grid: &Grid) -> u32 {
    let value = grid.data[pos.1][pos.0];
    // println!("{}", value);
    return value;
}

pub fn puzzle1() {
    println!("Day 15, puzzle 1");
    let start = Instant::now();
    let input = utils::file::read_input("src/day15/input.txt");

    let grid = create_grid(&input);
    let goal: Pos = Pos(grid.width - 1, grid.height - 1);

    let result = astar(
        &Pos(0, 0),
        |pos| successors(pos, &grid),
        |_| distance(&goal, &grid),
        |pos| *pos == goal,
    );

    match result {
        Some(path) => {
            // println!("Path: {:?}", path.0);
            println!("Risk level: {}", path.1);
        }
        None => println!("No path found"),
    }

    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    println!("Day 15, puzzle 2");
    let start = Instant::now();
    let input = utils::file::read_input("src/day15/input.txt");

    let grid = create_large_grid(&input);
    // let grid = create_grid(&input);
    let goal: Pos = Pos(grid.width - 1, grid.height - 1);

    let result = astar(
        &Pos(0, 0),
        |pos| successors(pos, &grid),
        |_| distance(&goal, &grid),
        |pos| *pos == goal,
    );

    match result {
        Some(path) => {
            // println!("Path: {:?}", path.0);
            println!("Risk level: {}", path.1);
        }
        None => println!("No path found"),
    }

    println!("Time elapsed: {:?}", start.elapsed());
}
