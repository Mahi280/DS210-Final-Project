use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, VecDeque};

pub fn get_or_add_node(graph: &mut UnGraph<String, ()>, label: String, node_map: &mut HashMap<String, NodeIndex>) -> NodeIndex {
    node_map.entry(label.clone()).or_insert_with(|| graph.add_node(label)).to_owned()
}

pub fn bfs_shortest_path(graph: &UnGraph<String, ()>, start_node: NodeIndex) -> HashMap<NodeIndex, usize> {
    let mut distances: HashMap<NodeIndex, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    distances.insert(start_node, 0);
    queue.push_back(start_node);

    while let Some(node) = queue.pop_front() {
        let current_distance = *distances.get(&node).unwrap_or(&0);
        for neighbor in graph.neighbors(node) {
            if distances.get(&neighbor).is_none() {
                distances.insert(neighbor, current_distance + 1);
                queue.push_back(neighbor);
            }
        }
    }

    distances
}

