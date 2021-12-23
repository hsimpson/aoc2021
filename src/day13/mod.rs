use crate::utils;
use std::time::Instant;

struct Paper {
    dots: Vec<Vec<char>>,
    width: u32,
    height: u32,
    folds: Vec<(char, u32)>,
}

fn create_paper(input: &String) -> Paper {
    let mut paper = Paper {
        dots: Vec::new(),
        width: 0,
        height: 0,
        folds: Vec::new(),
    };

    let mut coordinates: Vec<(u32, u32)> = Vec::new();

    for line in input.lines() {
        if line == "" {
            continue;
        }

        if line.starts_with("fold along") {
            let folding = line.replace("fold along ", "");
            let folds: Vec<&str> = folding.split("=").collect();

            paper.folds.push((
                folds[0].chars().next().unwrap(),
                folds[1].parse::<u32>().unwrap(),
            ));
        } else {
            let coords: Vec<&str> = line.split(",").collect();
            let x = coords[0].trim().parse::<u32>().unwrap();
            let y = coords[1].trim().parse::<u32>().unwrap();

            coordinates.push((x, y));

            if y + 1 > paper.height {
                paper.height = y + 1;
            }

            if x + 1 > paper.width {
                paper.width = x + 1;
            }
        }
    }

    // resize the dots vector
    paper.dots = vec![vec!['.'; paper.width as usize]; paper.height as usize];

    for coordinate in coordinates {
        let x = coordinate.0;
        let y = coordinate.1;

        paper.dots[y as usize][x as usize] = '#';
    }

    return paper;
}

fn print_paper(paper: &Paper) {
    println!();
    for row in paper.dots.iter() {
        for dot in row.iter() {
            print!("{}", dot);
        }
        println!();
    }
    println!();
}

fn fold_x(paper: &mut Paper, pos: u32) {
    let before = pos;
    let after = paper.width - pos - 1;

    let new_width = paper.width - (after + 1);

    let start_col = before - after;
    for row in 0..paper.dots.len() {
        let mut start_after_col = pos + after;
        for col in start_col..before {
            if paper.dots[row as usize][start_after_col as usize] == '#' {
                paper.dots[row as usize][col as usize] = '#';
            }
            start_after_col -= 1;
        }
        paper.dots[row].drain(pos as usize..);
    }

    paper.width = new_width;
}

fn fold_y(paper: &mut Paper, pos: u32) {
    let before = pos;
    let after = paper.height - pos - 1;

    let new_height = paper.height - (after + 1);

    let start_row = before - after;
    let mut start_after_row = pos + after;

    for row in start_row..before {
        for col in 0..paper.width {
            if paper.dots[start_after_row as usize][col as usize] == '#' {
                paper.dots[row as usize][col as usize] = '#';
            }
        }
        start_after_row -= 1;
        if start_after_row == pos {
            break;
        }
    }

    // remove the rows after pos
    paper.dots.drain(pos as usize..);

    paper.height = new_height;
}

fn count_dots(paper: &Paper) -> u32 {
    let mut count = 0;

    for row in paper.dots.iter() {
        for dot in row.iter() {
            if dot == &'#' {
                count += 1;
            }
        }
    }

    return count;
}

pub fn puzzle1() {
    println!("Day 13, puzzle 1");
    let start = Instant::now();
    let input = utils::file::read_input("src/day13/input.txt");

    let mut paper = create_paper(&input);
    for i in 0..1 {
        let fold = paper.folds[i];
        if fold.0 == 'x' {
            fold_x(&mut paper, fold.1);
        } else {
            fold_y(&mut paper, fold.1);
        }
    }

    println!("Visible dots: {}", count_dots(&paper));
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    println!("Day 13, puzzle 2");
    let start = Instant::now();
    let input = utils::file::read_input("src/day13/input.txt");

    let mut paper = create_paper(&input);
    for i in 0..paper.folds.len() {
        let fold = paper.folds[i];
        if fold.0 == 'x' {
            fold_x(&mut paper, fold.1);
        } else {
            fold_y(&mut paper, fold.1);
        }
    }

    print_paper(&paper);
    println!("Time elapsed: {:?}", start.elapsed());
}
