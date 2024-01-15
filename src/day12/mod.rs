use crate::utils;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Instant;

enum CaveType {
    Start,
    End,
    Small,
    Large,
}

struct Cave {
    id: usize,
    name: String,
    kind: CaveType,
    parents: Vec<usize>,
    children: Vec<usize>,
}

struct Graph {
    nodes: Vec<Cave>,
}

fn get_cave_type(input: &String) -> CaveType {
    if input == "start" {
        return CaveType::Start;
    } else if input == "end" {
        return CaveType::End;
    } else {
        let uppercase_input = input.clone().to_uppercase();
        if input == &uppercase_input {
            return CaveType::Large;
        } else {
            return CaveType::Small;
        }
    }
}

fn create_or_find_cave(graph: &mut Graph, name: &String) -> usize {
    let find_cave = graph.nodes.iter().position(|cave| cave.name == *name);
    if find_cave.is_some() {
        return find_cave.unwrap();
    } else {
        let cave_type = get_cave_type(name);
        let cave_id = graph.nodes.len();
        graph.nodes.push(Cave {
            id: cave_id,
            name: name.clone(),
            kind: cave_type,
            parents: Vec::new(),
            children: Vec::new(),
        });
        return cave_id;
    }
}

fn test_path(graph: &Graph, path: &mut Vec<usize>) {
    let mut visited: HashMap<usize, bool> = HashMap::new();
}

fn find_all_valid_paths(graph: &mut Graph, start_id: usize, end_id: usize) -> Vec<Vec<String>> {
    let mut valid_paths = Vec::new();
    let mut current_path: Vec<String> = Vec::new();
    let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
    queue.push_back(vec![start_id]);

    while queue.len() > 0 {}

    return valid_paths;
}

fn create_graph(input: &String) -> Graph {
    let mut graph = Graph { nodes: Vec::new() };

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let nodes = line.split("-").collect::<Vec<&str>>();
        let cave_name_left = nodes[0].to_string();
        let cave_name_right = nodes[1].to_string();

        let node_left = create_or_find_cave(&mut graph, &cave_name_left);
        let node_right = create_or_find_cave(&mut graph, &cave_name_right);

        graph.nodes[node_left].children.push(node_right);
        graph.nodes[node_right].parents.push(node_left);
    }

    return graph;
}

pub fn puzzle1() {
    println!("Day 12, puzzle 1");
    let start = Instant::now();
    let input = utils::file::read_input("src/day12/test.txt");

    let mut graph = create_graph(&input);
    let start_id = graph
        .nodes
        .iter()
        .position(|cave| cave.name == "start")
        .unwrap();
    let end_id = graph
        .nodes
        .iter()
        .position(|cave| cave.name == "end")
        .unwrap();
    let paths = find_all_valid_paths(&mut graph, start_id, end_id);

    println!("Valid paths: {:?}", paths.len());
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    println!("Day 12, puzzle ");
    let start = Instant::now();
    // let input = utils::file::read_input("src/day12/test.txt");

    println!("Time elapsed: {:?}", start.elapsed());
}
