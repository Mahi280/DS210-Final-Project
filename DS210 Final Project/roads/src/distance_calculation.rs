use petgraph::graph::{NodeIndex, UnGraph};
use rand::seq::SliceRandom;  
use rand::thread_rng;

pub fn calculate_average_shortest_path_length_sample(graph: &UnGraph<String, ()>, sample_size: usize) -> f64 {
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();
    let mut rng = thread_rng();

    let sample_nodes: Vec<NodeIndex> = node_indices
        .choose_multiple(&mut rng, sample_size)
        .cloned()
        .collect();

    let mut total_path_length = 0.0;
    let mut pair_count = 0;

    // Calculate shortest paths between sampled nodes
    for &start_node in &sample_nodes {
        let distances = crate::graph_utils::bfs_shortest_path(graph, start_node);
        for &end_node in &sample_nodes {
            if start_node != end_node {
                if let Some(distance) = distances.get(&end_node) {
                    total_path_length += *distance as f64;
                    pair_count += 1;
                }
            }
        }
    }

    if pair_count > 0 {
        total_path_length / pair_count as f64
    } else {
        0.0
    }
}

pub fn calculate_shortest_path_distance(graph: &UnGraph<String, ()>, start_node: usize, end_node: usize) -> Option<usize> {
    let start_index = NodeIndex::new(start_node);
    let end_index = NodeIndex::new(end_node);

    if start_index.index() >= graph.node_count() || end_index.index() >= graph.node_count() {
        return None;
    }

    let distances = crate::graph_utils::bfs_shortest_path(graph, start_index);

    distances.get(&end_index).cloned()
}
