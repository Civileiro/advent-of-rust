#![allow(dead_code)]

use itertools::Itertools;
use petgraph::{algo::dijkstra, graph::EdgeReference, prelude::*};

use std::{collections::HashMap, fmt::Debug, vec};

type MyGraphIx = u8;
type MyGraph = Graph<Valve, i32, Undirected, MyGraphIx>;

#[derive(Clone, PartialEq, Eq)]
struct Valve {
    name: String,
    flow_rate: i32,
}

impl Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.name, self.flow_rate))
    }
}

impl Valve {
    pub fn new(name: String, flow_rate: i32) -> Self {
        Self { name, flow_rate }
    }
}

#[derive(Debug)]
struct VolcanoPath<const M: i32> {
    visited: Vec<NodeIndex<MyGraphIx>>,
    minute: i32,
    pressure: i32,
    pressure_release: i32,
}

impl<const M: i32> VolcanoPath<M> {
    pub fn new(start: NodeIndex<MyGraphIx>) -> Self {
        Self {
            visited: vec![start],
            minute: 0,
            pressure: 0,
            pressure_release: 0,
        }
    }
    pub fn current(&self) -> NodeIndex<MyGraphIx> {
        *self.visited.last().unwrap()
    }
    pub fn possible_next_edges<'a>(&self, graph: &'a MyGraph) -> Vec<EdgeReference<'a, i32, u8>> {
        graph
            .edges(self.current())
            .filter(|e| e.weight() + self.minute < M)
            .filter(|e| !self.visited.contains(&e.target()))
            .collect_vec()
    }
    pub fn goto(&self, edge: EdgeReference<i32, u8>, graph: &MyGraph) -> Self {
        let mut visited = self.visited.clone();
        visited.push(edge.target());
        Self {
            visited,
            minute: edge.weight() + self.minute,
            pressure: self.pressure_release * edge.weight() + self.pressure,
            pressure_release: self.pressure_release + graph[edge.target()].flow_rate,
        }
    }
    pub fn final_pressure(&self) -> i32 {
        self.pressure + self.pressure_release * (M - self.minute)
    }
}

fn parse_input(input: &str) -> MyGraph {
    let parsed = input
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Valve ").unwrap();
            let (valve_name, line) = line.split_once(" has flow rate=").unwrap();
            let (flow_rate, line) = line.split_once("; tunnel").unwrap();
            let flow_rate = flow_rate.parse::<i32>().unwrap();
            let line = match line.strip_prefix('s') {
                Some(line) => line.strip_prefix(" lead to valves ").unwrap(),
                None => line.strip_prefix(" leads to valve ").unwrap(),
            };
            let connected = line.split(", ");
            (valve_name, flow_rate, connected)
        })
        .collect_vec();

    let mut valves = HashMap::<&str, _>::new();
    let mut raw_graph = Graph::default();
    for (valve_name, flow_rate, _) in &parsed {
        let node = raw_graph.add_node(Valve::new(valve_name.to_string(), *flow_rate));
        valves.insert(valve_name, node);
    }
    for (curr, _, connected) in parsed {
        for c in connected {
            raw_graph.update_edge(valves[curr], valves[c], 1);
        }
    }
    connect_positives(&raw_graph)
}

fn good_valve(valve: &Valve) -> bool {
    valve.flow_rate > 0 || valve.name == "AA"
}

fn connect_positives(original_graph: &MyGraph) -> MyGraph {
    let mut graph = Graph::default();
    for valve in original_graph.node_weights().filter(|v| good_valve(v)) {
        graph.add_node(valve.clone());
    }
    for start in original_graph
        .node_indices()
        .filter(|node| good_valve(&original_graph[*node]))
    {
        let node = graph
            .node_indices()
            .find(|n| graph[*n] == original_graph[start])
            .unwrap();
        let dij = dijkstra(original_graph, start, None, |_| 1);
        for (other, weight) in dij
            .into_iter()
            .filter(|(k, _)| good_valve(&original_graph[*k]))
        {
            let other_node = graph
                .node_indices()
                .find(|n| graph[*n] == original_graph[other])
                .unwrap();
            if node != other_node {
                graph.update_edge(node, other_node, weight + 1);
            }
        }
    }
    graph
}

fn find_optimal_path(graph: &MyGraph) -> VolcanoPath<30> {
    let start = graph
        .node_indices()
        .find(|i| graph[*i].name == "AA")
        .unwrap();
    let mut paths = vec![VolcanoPath::new(start)];
    let mut best_path: Option<VolcanoPath<30>> = None;
    while let Some(path) = paths.pop() {
        let next_edges = path.possible_next_edges(graph);
        if next_edges.is_empty() {
            // dbg!(&best_path, &path);
            best_path = match best_path {
                None => Some(path),
                Some(best) => {
                    if best.final_pressure() > path.final_pressure() {
                        Some(best)
                    } else {
                        Some(path)
                    }
                }
            };
            continue;
        }
        for edge in next_edges {
            paths.push(path.goto(edge, graph))
        }
    }
    best_path.unwrap()
}

pub fn day16_1(input: &str) -> i32 {
    let graph = parse_input(input);

    let best_path = find_optimal_path(&graph);
    best_path.final_pressure()
}

fn all_complete_paths(graph: &MyGraph) -> Vec<VolcanoPath<26>> {
    let start = graph
        .node_indices()
        .find(|i| graph[*i].name == "AA")
        .unwrap();
    let mut paths = vec![VolcanoPath::<26>::new(start)];
    let mut complete_paths = vec![];
    while let Some(path) = paths.pop() {
        let next_edges = path.possible_next_edges(graph);
        if next_edges.is_empty() {
            // dbg!(&best_path, &path);
            complete_paths.push(path);
            continue;
        }
        for edge in next_edges {
            paths.push(path.goto(edge, graph))
        }
    }
    complete_paths.sort_unstable_by_key(|p1| -p1.final_pressure());
    complete_paths
}

fn no_overlap(path: &VolcanoPath<26>, other: &VolcanoPath<26>) -> bool {
    let (_, other_visited_without_start) = other.visited.split_first().unwrap();
    path.visited
        .iter()
        .all(|p| !other_visited_without_start.iter().contains(p))
}

fn find_best_path_duo(paths: &[VolcanoPath<26>]) -> (&VolcanoPath<26>, &VolcanoPath<26>) {
    let (mut best, mut rest) = paths.split_first().unwrap();
    let mut best_duo = rest.iter().find(|p| no_overlap(best, p)).unwrap();
    let mut best_pressure = best.final_pressure() + best_duo.final_pressure();
    loop {
        let (path1, r) = rest.split_first().unwrap();
        rest = r;

        if 2 * path1.final_pressure() < best_pressure {
            return (best, best_duo);
        }
        let path2 = rest.iter().find(|p| no_overlap(path1, p)).unwrap();
        let this_pressure = path1.final_pressure() + path2.final_pressure();
        if this_pressure > best_pressure {
            best_pressure = this_pressure;
            best = path1;
            best_duo = path2;
        }
    }
}

pub fn day16_2(input: &str) -> i32 {
    let graph = parse_input(input);
    let paths = all_complete_paths(&graph);
    let (p1, p2) = find_best_path_duo(&paths);
    p1.final_pressure() + p2.final_pressure()
}

const _TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
