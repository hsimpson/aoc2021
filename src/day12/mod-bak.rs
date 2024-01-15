use crate::utils;
use petgraph::dot::{Config, Dot};
use petgraph::visit::Dfs;
use petgraph::Graph;
use std::fmt;
use std::time::Instant;

#[derive(Debug)]
enum CaveType {
    Start,
    End,
    Small,
    Large,
}

struct Cave {
    name: String,
    kind: CaveType,
}
impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_struct("Cave")
        //     .field("name", &self.name)
        //     .field("kind", &self.kind)
        //     .finish()
        write!(f, "{}", self.name)
    }
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

fn create_graph(input: &String) -> Graph<Cave, ()> {
    let mut graph = Graph::<Cave, ()>::new();

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let nodes = line.split("-").collect::<Vec<&str>>();
        let cave_name_left = nodes[0].to_string();
        let cave_name_right = nodes[1].to_string();

        // create or get left node
        let node_left;
        let find_left = graph
            .node_indices()
            .find(|i| graph[*i].name == cave_name_left);
        if find_left.is_some() {
            node_left = find_left.unwrap();
        } else {
            let cave_type = get_cave_type(&cave_name_left);
            node_left = graph.add_node(Cave {
                name: cave_name_left,
                kind: cave_type,
            });
        }

        // create or get right node
        let node_right;
        let find_right = graph
            .node_indices()
            .find(|i| graph[*i].name == cave_name_right);
        if find_right.is_some() {
            node_right = find_right.unwrap();
        } else {
            let cave_type = get_cave_type(&cave_name_right);
            node_right = graph.add_node(Cave {
                name: cave_name_right,
                kind: cave_type,
            });
        }

        graph.add_edge(node_left, node_right, ());
    }

    return graph;
}

pub fn puzzle1() {
    println!("Day 12, puzzle 1");
    let start = Instant::now();
    let input = utils::file::read_input("src/day12/test.txt");

    let graph = create_graph(&input);
    // get start node
    let start_node = graph
        .node_indices()
        .find(|i| graph[*i].name == "start")
        .unwrap();
    // let end_node = graph
    //     .node_indices()
    //     .find(|i| graph[*i].name == "end")
    //     .unwrap();

    let mut dfs = Dfs::new(&graph, start_node);
    while let Some(node) = dfs.next(&graph) {
        println!("{:?}", graph[node]);
    }

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    println!("Day 12, puzzle ");
    let start = Instant::now();
    // let input = utils::file::read_input("src/day12/test.txt");

    println!("Time elapsed: {:?}", start.elapsed());
}
