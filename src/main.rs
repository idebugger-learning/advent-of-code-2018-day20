use petgraph::algo::dijkstra;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;

mod map;
mod path;

fn main() {
    let input = include_str!("./data/input.txt").trim();
    // let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";

    println!("Building path...");
    let path = path::parse(&input);
    // println!("{:?}", Dot::with_config(&path, &[Config::EdgeNoLabel]));
    println!("Path built! It contains {} nodes and {} edges.", path.node_count(), path.edge_count());
    let has_cycles = petgraph::algo::is_cyclic_directed(&path);
    println!("Path has cycles: {}", has_cycles);
    // println!("Building map...");
    // let map = map::Map::new(&path);
    // println!("Map built! Printing map...");
    // println!("{}", map);
    let path_lengths = dijkstra(&path, NodeIndex::new(0), None, |_| 1);
    let longest_path = path_lengths.iter().max_by_key(|(_, length)| *length).unwrap();
    println!("Longest path: {:?}", longest_path);
}
