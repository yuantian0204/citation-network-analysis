use crate::centrality::degree_centrality::calculate_degree_centrality;
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
    println!("{:?}", network);
    let ranks = calculate_degree_centrality(&network);
    println!("{}", ranks.top(10));
}
