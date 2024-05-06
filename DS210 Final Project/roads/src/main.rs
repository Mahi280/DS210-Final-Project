mod graph_utils;
mod file_utils;
mod distance_calculation;
mod test;

use file_utils::*;
use distance_calculation::*;
use std::io;
use std::str::FromStr;

fn main() {
    let file_path = "/Users/mahisaboo/Downloads/DS210 Final Project/roads/roadNet-TX_2.csv";

    if let Ok(graph) = read_graph_from_csv(file_path) {
        println!("Number of nodes: {}", graph.node_count());
        println!("Number of edges: {}", graph.edge_count());

        let sample_avg_shortest_path_length = calculate_average_shortest_path_length_sample(&graph, 1000);
        println!("Sample Average Shortest Path Length: {:.2}", sample_avg_shortest_path_length);

        loop {
            println!("Enter two node indices to find the shortest path (e.g., '5 10'), or type 'exit' to quit:");
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Failed to read input.");
                continue;
            }

            if input.trim().eq("exit") {
                break;
            }

            let nodes: Vec<usize> = input.trim().split_whitespace()
                .filter_map(|num| usize::from_str(num).ok())
                .collect();

            if nodes.len() == 2 {
                match crate::distance_calculation::calculate_shortest_path_distance(&graph, nodes[0], nodes[1]) {
                    Some(distance) => println!("The shortest path distance between nodes {} and {} is: {}", nodes[0], nodes[1], distance),
                    None => println!("One or both node indices are out of bounds, or no path exists between them."),
                }
            } else {
                println!("Please enter exactly two node indices.");
            }
        }

    } else {
        println!("Error reading the file");
    }
}



