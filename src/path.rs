use petgraph::Graph;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    Start,
    End,
    Direction(Direction),
}

pub type Path = Graph<NodeType, ()>;

pub fn parse(input: &str) -> Path {
    let mut graph = Graph::new();
    let mut last_indexes = vec![graph.add_node(NodeType::Start)];
    let mut split_points = Vec::new();
    let mut merge_points = Vec::new();

    for (index, char) in input.chars().enumerate() {
        match char {
            '^' => {},
            '$' => {
                let inserted = graph.add_node(NodeType::End);
                for last_index in &last_indexes {
                    graph.add_edge(*last_index, inserted, ());
                }
            },
            'N' => {
                let inserted = graph.add_node(NodeType::Direction(Direction::North));
                for last_index in &last_indexes {
                    graph.add_edge(*last_index, inserted, ());
                }
                last_indexes = vec![inserted];
            },
            'S' => {
                let inserted = graph.add_node(NodeType::Direction(Direction::South));
                for last_index in &last_indexes {
                    graph.add_edge(*last_index, inserted, ());
                }
                last_indexes = vec![inserted];
            },
            'E' => {
                let inserted = graph.add_node(NodeType::Direction(Direction::East));
                for last_index in &last_indexes {
                    graph.add_edge(*last_index, inserted, ());
                }
                last_indexes = vec![inserted];
            },
            'W' => {
                let inserted = graph.add_node(NodeType::Direction(Direction::West));
                for last_index in &last_indexes {
                    graph.add_edge(*last_index, inserted, ());
                }
                last_indexes = vec![inserted];
            },
            '(' => {
                split_points.push(last_indexes.clone());
                merge_points.push(vec![]);
            },
            '|' => {
                let last_merge_points = merge_points.last_mut().unwrap();
                last_merge_points.extend(last_indexes);
                last_indexes = split_points.last().unwrap().clone();
            },
            ')' => {
                split_points.pop();

                let mut merge = merge_points.pop().unwrap();
                merge.extend(last_indexes);

                last_indexes = merge;
            }
            _ => panic!("Unknown character: {}", char),
        }
    }

    graph
}
