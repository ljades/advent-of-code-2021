use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code_2021::inputs::read_input;

enum CaveType {
    Large,
    Small,
    StartPoint,
    EndPoint,
}

impl From<&String> for CaveType {
    fn from(cave_name: &String) -> Self {
        if cave_name == "end" {
            CaveType::EndPoint
        } else if cave_name == "start" {
            CaveType::StartPoint
        }
        else if cave_name.chars().any(|c| c.is_ascii_lowercase()) {
            CaveType::Small
        } else {
            CaveType::Large
        }
    }
}

const START_POINT: &str = "start";

struct PathSet {
    path: Vec<String>,
    small_caves_visited: HashSet<String>,
    has_double_dipped: bool,
    current_cave: String,
}

impl PathSet {
    fn new() -> PathSet {
        let start = String::from(START_POINT);
        PathSet {
            path: vec![start.clone()],
            small_caves_visited: HashSet::from([start.clone()]),
            has_double_dipped: false,
            current_cave: start.clone(),
        }
    }

    fn copy_with_insert(&self, latest_cave: &String) -> PathSet {
        // Return a value of a new PathSet that includes the latest cave passed in
        let mut path = self.path.clone();
        path.push(latest_cave.clone());
        let mut small_caves_visited = self.small_caves_visited.clone();
        let mut has_double_dipped = self.has_double_dipped;
        match CaveType::from(latest_cave) {
            CaveType::Small => {
                if !small_caves_visited.insert(latest_cave.clone()) { has_double_dipped = true; }
            },
            _ => (),
        }
        let current_cave = latest_cave.clone();
        PathSet{ path, small_caves_visited, has_double_dipped, current_cave }
    }
}

struct CaveGraph {
    edges: HashMap<String, HashSet<String>>,
}

impl From<Vec<String>> for CaveGraph {
    fn from(edge_strings: Vec<String>) -> Self {
        let mut all_edges: HashMap<String, HashSet<String>> = HashMap::new();

        for edge_string in edge_strings.iter() {
            let mut edge_string_split = edge_string.split("-");
            let cave_1 = edge_string_split.next().unwrap().to_string();
            let cave_2 = edge_string_split.next().unwrap().to_string();
            match all_edges.get(&cave_1) {
                Some(cave_edge_set) => {
                    let mut cave_edge_set_updated = cave_edge_set.clone();
                    cave_edge_set_updated.insert(cave_2.clone());
                    all_edges.insert(cave_1.clone(), cave_edge_set_updated);
                },
                None => {
                    all_edges.insert(cave_1.clone(), HashSet::from([cave_2.clone()]));
                },
            }
            match all_edges.get(&cave_2) {
                Some(cave_edge_set) => {
                    let mut cave_edge_set_updated = cave_edge_set.clone();
                    cave_edge_set_updated.insert(cave_1.clone());
                    all_edges.insert(cave_2.clone(), cave_edge_set_updated);
                },
                None => {
                    all_edges.insert(cave_2.clone(), HashSet::from([cave_1.clone()]));
                },
            }
        }

        CaveGraph{ edges: all_edges }
    }
}

impl CaveGraph {
    fn find_all_paths(&self, with_double_dip: bool) -> Vec<PathSet> {
        let mut search_stack: Vec<PathSet> = vec![
            PathSet::new()
        ];
        let mut valid_path_sets: Vec<PathSet> = Vec::new();
        while let Some(search_vertex) = search_stack.pop() {
            match self.edges.get(&search_vertex.current_cave) {
                Some(connecting_caves) => {
                    for connecting_cave in connecting_caves.iter() {
                        match CaveType::from(connecting_cave) {
                            CaveType::EndPoint => {
                                let valid_path_set = search_vertex.copy_with_insert(&connecting_cave);
                                valid_path_sets.push(valid_path_set);
                            },
                            CaveType::StartPoint => (),
                            _ => {
                                if !search_vertex.small_caves_visited.contains(connecting_cave) || (
                                    with_double_dip && !search_vertex.has_double_dipped
                                ) {
                                    let next_search = search_vertex.copy_with_insert(&connecting_cave);
                                    search_stack.push(next_search);
                                }
                            },
                        }
                    }
                }
                None => panic!("we should not have gotten here! no edges for this cave?")
            }
        }

        valid_path_sets
    }
}

fn get_cave_graph(file_contents: String) -> CaveGraph {
    let file_as_string_vec: Vec<String> = file_contents.split("\n").map(|s| s.to_string()).collect();
    CaveGraph::from(file_as_string_vec)
}

fn solve_part_1(cave_graph: &CaveGraph) -> usize {
    cave_graph.find_all_paths(false).len()
}

fn solve_part_2(cave_graph: &CaveGraph) -> usize {
    cave_graph.find_all_paths(true).len()
}

fn main() {
    let problem_raw_input = read_input("src/inputs/i12.txt");
    let cave_graph = get_cave_graph(problem_raw_input);
    println!("{}", solve_part_1(&cave_graph));
    println!("{}", solve_part_2(&cave_graph));
}
