use std::fs::File;
use crate::network::CitationNetwork;

mod network;

fn main() {
    let file = File::open("data/cit-HepTh.txt").unwrap();
    let network = CitationNetwork::load_from_file(file);
    println!("{:?}", network);
}
