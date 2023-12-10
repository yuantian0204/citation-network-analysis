use crate::centrality::degree_centrality::calculate_degree_centrality;
use crate::centrality::pagerank_centrality::calculate_pagerank_centrality;
use std::fs::File;

use crate::network::CitationNetwork;

mod network;

mod centrality {
    pub(crate) mod centrality;
    pub(crate) mod degree_centrality;
    pub(crate) mod pagerank_centrality;
}

fn main() {
    let file = File::open("data/cit-HepTh.txt").unwrap();
    let network = CitationNetwork::load_from_file(file);
    let degree_ranks = calculate_degree_centrality(&network);
    println!("Degree Centrality Scores: \n{}", degree_ranks.top(5));
    let pagerank_ranks = calculate_pagerank_centrality(&network);
    println!("PageRank Centrality Scores: \n{}", pagerank_ranks.top(5));
}
