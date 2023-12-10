use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// A network of citations
pub struct CitationNetwork {
    /// The graph is stored as adjacency lists for each node
    edges: HashMap<usize, Vec<usize>>,
}

impl CitationNetwork {
    /// Creates a new empty network
    fn new() -> CitationNetwork {
        CitationNetwork {
            edges: HashMap::new()
        }
    }
    /// Adds an edge to the network
    ///
    /// An edge from i to j represents a citation to paper j in paper i.
    ///
    /// # Arguments
    ///
    /// * `from` - The id of the source paper
    /// * `to` - The id of the cited paper
    ///
    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_insert_with(Vec::new).push(to);
        self.edges.entry(to).or_insert_with(Vec::new);
    }
    /// Returns the number of nodes in the network
    fn size(&self) -> usize {
        return self.edges.len();
    }
    /// Returns the number of edges in the network
    fn num_edges(&self) -> usize {
        return self.edges.values().map(|x| x.len()).sum();
    }
    /// Loads a network from a file
    ///
    /// # Arguments
    ///
    /// * `file` - The file to load from
    ///
    pub fn load_from_file(file: File) -> CitationNetwork {
        let reader = BufReader::new(file);
        let mut graph = CitationNetwork::new();
        for line in reader.lines().skip(4) {
            let line = line.unwrap();
            let entries: Vec<usize> = line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap()).collect();
            graph.add_edge(entries[0], entries[1]);
        }
        graph
    }
}

impl Debug for CitationNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nodes: {} Edges: {}", self.size(), self.num_edges())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut graph = CitationNetwork::new();
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);
        assert_eq!(graph.size(), 4);
        assert!(graph.edges.contains_key(&0));
        assert_eq!(graph.edges[&0], vec![1, 2, 3]);
        assert!(graph.edges.contains_key(&1));
        assert_eq!(graph.edges[&1], vec![2, 3]);
        assert!(graph.edges.contains_key(&2));
        assert_eq!(graph.edges[&2], vec![3]);
        assert!(graph.edges.contains_key(&3));
        assert_eq!(graph.edges[&3], vec![]);
        assert_eq!(graph.num_edges(), 6);
    }

    #[test]
    fn test_load_network_from_file() {
        let file = File::open("data/cit-HepTh.txt").unwrap();
        let network = CitationNetwork::load_from_file(file);
        assert_eq!(network.size(), 27770);
        assert_eq!(network.num_edges(), 352807);
    }
}