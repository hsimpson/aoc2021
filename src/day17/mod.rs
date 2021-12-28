use std::time::Instant;

struct Area {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

struct Position {
    x: i32,
    y: i32,
}

type Velocity = Position;

enum IntersectionType {
    Before,
    Hit,
    After,
}

fn create_area() -> Area {
    // test data
    // Area {
    //     x_min: 20,
    //     x_max: 30,
    //     y_min: -10,
    //     y_max: -5,
    // }

    // real data
    Area {
        x_min: 143,
        x_max: 177,
        y_min: -106,
        y_max: -71,
    }
}

fn intersect(pos: &Position, area: &Area) -> IntersectionType {
    if pos.x > area.x_min + area.x_max || pos.y < area.y_min {
        return IntersectionType::After;
    }

    if pos.x >= area.x_min && pos.x <= area.x_max && pos.y <= area.y_max && pos.y >= area.y_min {
        return IntersectionType::Hit;
    }
    return IntersectionType::Before;
}

fn shoot(area: &Area, vel: Velocity, max_y: &mut i32) -> bool {
    let mut position = Position { x: 0, y: 0 };
    let mut velocity = vel;

    loop {
        position.x = position.x + velocity.x;
        position.y = position.y + velocity.y;

        // println!("Pos: {}, {}", position.x, position.y);

        if position.y > *max_y {
            *max_y = position.y;
        }
        match intersect(&position, area) {
            IntersectionType::Hit => {
                return true;
            }
            IntersectionType::After => {
                return false;
            }
            _ => {}
        }

        if velocity.x > 0 {
            velocity.x = velocity.x - 1;
        }
        velocity.y = velocity.y - 1;
    }
}

fn get_highest_y(area: &Area) -> i32 {
    let mut max_y = 0;
    for x in 1..1000 {
        for y in -1000..1000 {
            let mut current_max_y = 0;
            if shoot(&area, Position { x: x, y: y }, &mut current_max_y) {
                max_y = std::cmp::max(max_y, current_max_y);
            }
        }
    }

    return max_y;
}

fn hitting_velosities(area: &Area) -> u32 {
    let mut velocities = 0;
    for x in 1..1000 {
        for y in -1000..1000 {
            let mut current_max_y = 0;
            if shoot(&area, Position { x: x, y: y }, &mut current_max_y) {
                velocities = velocities + 1;
            }
        }
    }

    return velocities;
}

pub fn puzzle1() {
    println!("Day 17, puzzle 1");
    let start = Instant::now();

    let area = create_area();
    let max_y = get_highest_y(&area);
    println!("Highest y: {}", max_y);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    println!("Day 17, puzzle 2");
    let start = Instant::now();

    let area = create_area();
    let velocities = hitting_velosities(&area);
    println!("Velocities: {}", velocities);
    println!("Time elapsed: {:?}", start.elapsed());
}
