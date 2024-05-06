use csv::ReaderBuilder;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Error, ErrorKind};

use crate::graph_utils::get_or_add_node;

pub fn read_graph_from_csv(file_path: &str) -> io::Result<UnGraph<String, ()>> {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    let file = File::open(file_path).map_err(|_| Error::new(ErrorKind::NotFound, "Failed to open the file"))?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    for result in reader.records() {
        let record = result.map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to parse CSV record"))?;
        if record.len() >= 2 {
            let source = get_or_add_node(&mut graph, record[0].to_string(), &mut node_map);
            let target = get_or_add_node(&mut graph, record[1].to_string(), &mut node_map);
            graph.add_edge(source, target, ());
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "CSV record must have at least two columns"));
        }
    }

    Ok(graph)
}
