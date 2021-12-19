use crate::utils;
use std::time::Instant;

struct Point {
    x: usize,
    y: usize,
    value: u32,
}

struct TrackedPoint {
    point: Point,
    is_tracked: bool,
}

struct Grid {
    points: Vec<Vec<TrackedPoint>>,
    width: usize,
    height: usize,
}

struct Basin {
    sum: u32,
}

fn build_grid(lines: &Vec<String>) -> Grid {
    let mut grid: Grid = Grid {
        points: Vec::new(),
        width: 0,
        height: 0,
    };

    for (i, line) in lines.iter().enumerate() {
        let numbers: Vec<char> = line.chars().collect();
        let line_numbers: Vec<u32> = numbers.iter().map(|c| c.to_digit(10).unwrap()).collect();
        let mut grid_line: Vec<TrackedPoint> = Vec::new();
        for (j, number) in line_numbers.iter().enumerate() {
            let point = Point {
                x: j,
                y: i,
                value: *number,
            };
            let tracked_point = TrackedPoint {
                point,
                is_tracked: false,
            };
            grid_line.push(tracked_point);
            grid.width += 1;
        }
        grid.points.push(grid_line);
    }

    grid.width = lines[0].len();
    grid.height = lines.len();

    return grid;
}

fn find_low_points(grid: &Grid) -> Vec<Point> {
    let mut low_points: Vec<Point> = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let number = grid.points[y][x].point.value;
            let mut up = 9;
            let mut right = 9;
            let mut down = 9;
            let mut left = 9;

            // check up
            if y > 0 {
                up = grid.points[y - 1][x].point.value;
            }

            // check right
            if x < grid.width - 1 {
                right = grid.points[y][x + 1].point.value;
            }

            // check down
            if y < grid.height - 1 {
                down = grid.points[y + 1][x].point.value;
            }

            // check left
            if x > 0 {
                left = grid.points[y][x - 1].point.value;
            }

            if number < up && number < right && number < down && number < left {
                low_points.push(Point {
                    x: x,
                    y: y,
                    value: number,
                });
            }
        }
    }

    return low_points;
}

fn fill_basin(x: usize, y: usize, basin: &mut Basin, grid: &mut Grid) {
    let tracked_point = &grid.points[y][x];
    if tracked_point.is_tracked || tracked_point.point.value == 9 {
        return;
    }

    basin.sum += 1;
    grid.points[y][x].is_tracked = true;

    // top
    if y > 0 {
        fill_basin(x, y - 1, basin, grid);
    }

    // right
    if x < grid.width - 1 {
        fill_basin(x + 1, y, basin, grid);
    }

    // bottom
    if y < grid.height - 1 {
        fill_basin(x, y + 1, basin, grid);
    }

    // left
    if x > 0 {
        fill_basin(x - 1, y, basin, grid);
    }
}

fn create_basins(low_points: &Vec<Point>, grid: &mut Grid) -> Vec<Basin> {
    let mut basins: Vec<Basin> = Vec::new();

    for point in low_points {
        let mut basin: Basin = Basin { sum: 0 };

        fill_basin(point.x, point.y, &mut basin, grid);
        basins.push(basin);
    }

    return basins;
}

pub fn puzzle1() {
    println!("Day 9, puzzle 1");
    let start = Instant::now();
    let input = utils::file::read_input("src/day9/input.txt");
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let grid = build_grid(&lines);

    let low_points = find_low_points(&grid);
    // accumulate the low points values, adding 1 to each value
    let sum = low_points
        .iter()
        .fold(0, |acc, low_point| acc + low_point.value + 1);

    println!("Sum: {}", sum);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    println!("Day 9, puzzle 2");
    let start = Instant::now();
    let input = utils::file::read_input("src/day9/input.txt");
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let mut grid = build_grid(&lines);

    let low_points = find_low_points(&grid);
    let mut basins = create_basins(&low_points, &mut grid);

    if basins.len() >= 3 {
        basins.sort_by(|a, b| b.sum.cmp(&a.sum));
        let result = basins[0].sum * basins[1].sum * basins[2].sum;
        println!("Result: {}", result);
    }

    println!("Time elapsed: {:?}", start.elapsed());
}
