use petgraph::algo::dijkstra;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;

mod map;
mod path;

fn main() {
    let input = include_str!("./data/input.txt").trim();
    // let input = include_str!("./data/input_reduced.txt").trim();
    // let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
    // let input = "^(SSS|EEESSSWWW)ENNES$";

    println!("Building path...");
    let path = path::parse(&input);
    // println!("{:?}", Dot::with_config(&path, &[Config::EdgeNoLabel]));
    println!("Path built! It contains {} nodes and {} edges.", path.node_count(), path.edge_count());
    let has_cycles = petgraph::algo::is_cyclic_directed(&path);
    println!("Path has cycles: {}", has_cycles);

    // let end_node = path.node_indices().find(|node| path[*node] == path::NodeType::End).unwrap();
    // println!("Finding all simple paths...");
    // let all_simple_paths = petgraph::algo::all_simple_paths::<Vec<NodeIndex>, _>(&path, NodeIndex::new(0), end_node, 0, None).count();
    // println!("All simple paths: {}", all_simple_paths);

    println!("Building map...");
    let map = map::Map::new(&path);
    println!("Map built! It contains {} rooms.", map.rooms.len());
    // println!("{}", map);

    let map_graph = map.to_graph();
    println!("Map graph built! It contains {} nodes and {} edges.", map_graph.node_count(), map_graph.edge_count());
    let has_cycles = petgraph::algo::is_cyclic_undirected(&map_graph);
    println!("Map graph has cycles: {}", has_cycles);
    let zero_index = map_graph.node_indices().find(|node| map_graph[*node] == (0, 0)).unwrap();
    println!("NodeIndex of (0, 0) is {:?}", zero_index);
    let path_lengths = dijkstra(&map_graph, zero_index, None, |_| 1);
    // println!("Path lengths: {:?}", path_lengths);
    let longest_path = path_lengths.iter().max_by_key(|(_, length)| *length).unwrap();
    println!("Longest path: {:?}", longest_path);

    let paths_longer_than_1000 = path_lengths.iter().filter(|(_, length)| *length >= &1000).count();
    println!("Paths longer than 1000: {}", paths_longer_than_1000);

    // println!("{:?}", Dot::with_config(&map_graph, &[Config::EdgeNoLabel]));
}
