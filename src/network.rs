use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// A network of citations
pub(crate) struct CitationNetwork {
    /// The graph is stored as adjacency lists for each node
    in_edges: HashMap<usize, Vec<usize>>,
}

impl CitationNetwork {
    /// Creates a new empty network
    pub(crate) fn new() -> CitationNetwork {
        CitationNetwork {
            in_edges: HashMap::new(),
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
    pub(crate) fn add_edge(&mut self, from: usize, to: usize) {
        self.in_edges.entry(to).or_insert_with(Vec::new).push(from);
        self.in_edges.entry(from).or_insert_with(Vec::new);
    }
    /// Returns the number of nodes in the network
    pub(crate) fn size(&self) -> usize {
        return self.in_edges.len();
    }
    /// Returns the number of edges in the network
    pub(crate) fn num_edges(&self) -> usize {
        return self.in_edges.values().map(|x| x.len()).sum();
    }
    /// Returns the nodes in the network
    pub(crate) fn nodes(&self) -> impl Iterator<Item = &usize> {
        self.in_edges.keys()
    }
    /// Returns an iterator over the edges in the network
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&usize, &Vec<usize>)> {
        self.in_edges.iter()
    }
    /// Returns the in-edges to a given vertex in the network
    pub(crate) fn edges(&self, vertex: usize) -> impl Iterator<Item = &usize> {
        self.in_edges[&vertex].iter()
    }
    /// Loads a network from a file
    ///
    /// # Arguments
    ///
    /// * `file` - The file to load from
    ///
    pub(crate) fn load_from_file(file: File) -> CitationNetwork {
        let reader = BufReader::new(file);
        let mut graph = CitationNetwork::new();
        for line in reader.lines().skip(4) {
            let line = line.unwrap();
            let entries: Vec<usize> = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
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
        assert!(graph.in_edges.contains_key(&0));
        assert_eq!(graph.in_edges[&0], vec![]);
        assert!(graph.in_edges.contains_key(&1));
        assert_eq!(graph.in_edges[&1], vec![0]);
        assert!(graph.in_edges.contains_key(&2));
        assert_eq!(graph.in_edges[&2], vec![0, 1]);
        assert!(graph.in_edges.contains_key(&3));
        assert_eq!(graph.in_edges[&3], vec![0, 1, 2]);
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
