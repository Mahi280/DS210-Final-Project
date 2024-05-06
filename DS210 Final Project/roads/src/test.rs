#[cfg(test)]
mod tests {
    use crate::distance_calculation::{calculate_average_shortest_path_length_sample, calculate_shortest_path_distance};
    use petgraph::graph::UnGraph;

    #[test]
    fn test_average_shortest_path_length_sample() {
        let mut graph = UnGraph::<String, ()>::new_undirected();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());
        let d = graph.add_node("D".to_string());
    
        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());
        graph.add_edge(c, d, ());
    
        let sample_size = 4;  
        let avg_length = calculate_average_shortest_path_length_sample(&graph, sample_size);
    
        let expected_paths = [1, 1, 1, 2, 2, 3];
        let expected_avg_length = expected_paths.iter().sum::<usize>() as f64 / expected_paths.len() as f64;
    
        assert!(
            (avg_length - expected_avg_length).abs() < f64::EPSILON,
            "Expected average length not met, got {}, expected {}", avg_length, expected_avg_length
        );
    }

    #[test]
    fn test_calculate_shortest_path_distance() {
        let mut graph = UnGraph::<String, ()>::new_undirected();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());

        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());

        let start_node = a.index();
        let end_node = c.index();

        let distance = calculate_shortest_path_distance(&graph, start_node, end_node);
        assert_eq!(distance, Some(2), "Expected distance not met between nodes A and C");
    }
}