use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::centrality::centrality::{Centrality, CentralityRank};
use crate::network::CitationNetwork;

/// The PageRank of a single node in the network.
#[derive(Clone)]
pub(crate) struct PageRankCentrality {
    vertex: usize,
    pagerank: f64,
}

impl PageRankCentrality {
    fn new(vertex: usize, pagerank: f64) -> PageRankCentrality {
        PageRankCentrality { vertex, pagerank }
    }
}

impl PartialOrd for PageRankCentrality {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pagerank.partial_cmp(&other.pagerank)
    }
}

const EPSILON: f64 = 1e-12;

impl PartialEq<Self> for PageRankCentrality {
    fn eq(&self, other: &Self) -> bool {
        (self.pagerank - other.pagerank).abs() <= EPSILON
    }
}

impl Display for PageRankCentrality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "vertex {}: PageRank {}", self.vertex, self.pagerank)
    }
}

impl Centrality<f64> for PageRankCentrality {
    fn vertex(&self) -> usize {
        self.vertex
    }

    fn score(&self) -> f64 {
        self.pagerank
    }
}

const DAMPING_FACTOR: f64 = 0.85;
const MAX_ITERATIONS: usize = 100;
const TOLERANCE: f64 = 1e-9;

/// Performs one iteration of the PageRank algorithm.
///
/// # Arguments
///
/// * `network` - The network to analyze
/// * `page_ranks` - The PageRank scores of the network
///
/// # Returns
///
/// * `converged` - Whether or not this iteration has converged
fn pagerank_iterate(network: &CitationNetwork, page_ranks: &mut HashMap<usize, f64>) -> bool {
    let mut new_page_ranks: HashMap<usize, f64> = HashMap::new();
    let num_nodes = network.size();
    let mut delta = 0.0; // used to check convergence
    let mut sink_node_contributions: f64 = 0.0; // Handle sink nodes
    for &vertex in network.nodes() {
        if network.out_edges_from(vertex).count() == 0 {
            sink_node_contributions += page_ranks.get(&vertex).unwrap_or(&0.0);
        }
    }
    sink_node_contributions /= num_nodes as f64;
    // Update the PageRank scores
    for &vertex in network.nodes() {
        let mut sum = 0.0;
        for &in_edge in network.in_edges_to(vertex) {
            sum += page_ranks.get(&in_edge).unwrap_or(&0.0)
                / network.out_edges_from(in_edge).count() as f64;
        }
        sum += sink_node_contributions;
        sum = (1.0 - DAMPING_FACTOR) / (num_nodes as f64) + DAMPING_FACTOR * sum;
        delta += (sum - page_ranks.get(&vertex).unwrap_or(&0.0)).abs();
        new_page_ranks.insert(vertex, sum);
    }
    *page_ranks = new_page_ranks;
    delta < TOLERANCE
}

/// Returns the PageRank centrality scores of a network
///
/// The PageRank measures the relative importance of a node in the network. It is
/// computed using an iterative algorithm.
///
/// # Arguments
///
/// * `network` - The network to analyze
pub(crate) fn calculate_pagerank_centrality(
    network: &CitationNetwork,
) -> CentralityRank<f64, PageRankCentrality> {
    let mut page_ranks: HashMap<usize, f64> = HashMap::new();
    for &vertex in network.nodes() {
        page_ranks.insert(vertex, 1.0 / (network.size() as f64));
    }
    let mut converged = false;
    let mut num_iterations = 0;
    while !converged && num_iterations < MAX_ITERATIONS {
        converged = pagerank_iterate(network, &mut page_ranks);
        num_iterations += 1;
    }
    // Convert the HashMap to a sorted vector
    let mut ranks: Vec<_> = page_ranks
        .into_iter()
        .map(|(vertex, rank)| PageRankCentrality::new(vertex, rank))
        .collect();
    ranks.sort_by(|a, b| b.partial_cmp(a).unwrap());
    CentralityRank::new(ranks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_pagerank_centrality() {
        let mut network = CitationNetwork::new();
        network.add_edge(0, 1);
        network.add_edge(0, 2);
        network.add_edge(1, 2);
        let pagerank_ranks = calculate_pagerank_centrality(&network);
        println!("{}", pagerank_ranks);
        assert_eq!(pagerank_ranks[0].vertex(), 2);
        assert!((pagerank_ranks[0].score() - 0.521).abs() < 0.001);
        assert_eq!(pagerank_ranks[1].vertex(), 1);
        assert!((pagerank_ranks[1].score() - 0.281).abs() < 0.001);
        assert_eq!(pagerank_ranks[2].vertex(), 0);
        assert!((pagerank_ranks[2].score() - 0.198).abs() < 0.001);
    }
}
