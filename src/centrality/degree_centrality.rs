use crate::centrality::centrality::{Centrality, CentralityRank};
use crate::network::CitationNetwork;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// The in-degree centrality score of a single paper
///
/// The in-degree centrality score of a paper represents the number of times
/// it is cited by other papers in the network.
#[derive(Clone)]
pub(crate) struct DegreeCentrality {
    vertex: usize,
    in_degree: i32,
}

impl DegreeCentrality {
    fn new(vertex: usize, in_degree: i32) -> DegreeCentrality {
        DegreeCentrality { vertex, in_degree }
    }
}

impl PartialOrd for DegreeCentrality {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.in_degree.partial_cmp(&other.in_degree)
    }
}

impl PartialEq<Self> for DegreeCentrality {
    fn eq(&self, other: &Self) -> bool {
        self.in_degree == other.in_degree
    }
}

impl Display for DegreeCentrality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "vertex {}: in-degree {}", self.vertex, self.in_degree)
    }
}

impl Centrality<i32> for DegreeCentrality {
    fn vertex(&self) -> usize {
        self.vertex
    }

    fn score(&self) -> i32 {
        self.in_degree
    }
}

/// Calculates the in-degree centrality scores of a network
///
/// # Arguments
///
/// * `network` - The network to analyze
pub(crate) fn calculate_degree_centrality(
    network: &CitationNetwork,
) -> CentralityRank<i32, DegreeCentrality> {
    let mut ranks: Vec<_> = network
        .iter()
        .map(|(&vertex, edges)| DegreeCentrality::new(vertex, edges.len() as i32))
        .collect();
    ranks.sort_by(|a, b| b.partial_cmp(a).unwrap());
    CentralityRank::new(ranks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_degree_centrality() {
        let mut graph = CitationNetwork::new();
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);
        let ranks = calculate_degree_centrality(&graph);
        assert_eq!(ranks[0].vertex(), 3);
        assert_eq!(ranks[0].score(), 3);
        assert_eq!(ranks[1].vertex(), 2);
        assert_eq!(ranks[1].score(), 2);
        assert_eq!(ranks[2].vertex(), 1);
        assert_eq!(ranks[2].score(), 1);
        assert_eq!(ranks[3].vertex(), 0);
        assert_eq!(ranks[3].score(), 0);
    }
}
