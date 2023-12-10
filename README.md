# Citation Network Analysis
Analyzing the citation network to identify the most influential articles.

## Motivation

In scientific research, we often try to find the most influential papers in a field of study. Typically, the importance 
of a study can be estimated by counting the number of times it is cited in other papers. However, this approach may not 
be the most accurate. If we model the entire body of literature in a field as a graph, where each vertex is a study, 
and each edge refers to a citation, we can obtain a more accurate estimate of a paper’s importance by computing its 
centrality in the citation network. Furthermore, by analyzing different centrality measures, we can study the different 
ways a paper can make an impact on the field.

## Data Set

The data set used in this project is the [High-energy physics theory citation network](https://snap.stanford.edu/data/cit-HepTh.html)
obtained from the [Stanford Network Analysis Project](https://snap.stanford.edu/index.html). The downloaded data file 
can be found in ```data```, where ```data/cit-HepTh.txt``` contains all the edges in the citation network. The ID of 
each node is the ID of the corresponding paper in the arxiv database. For example, node 9207016 refers to [Noncompact Symmetries in String Theory](https://arxiv.org/abs/hep-th/9207016).

## Methods

This project analyzes two centrality measures: degree centrality and PageRank. Since the citation network is sparse, we
use adjacency lists to store the graph. For the purpose of time efficiency, the ```CitationNetowrk``` stores both
incoming and outgoing edges for each node.

The degree centrality simply refers to the number of incoming edges to each node, which corresponds to the number of
times a paper is cited. This is a straightforward way of evaluating the importance of an article, but it does not
take into account the influence an article has beyond direct citations.

The PageRank algorithm is used by Google to rank web pages in their search results. It measures the relative importance
of a node in a network by weighting the incoming edges by their importance. In other words, a paper with higher rank
should contribute more to the rank of the other papers it cites. In this project, the PageRank centrality is computed 
using the [iterative algorithm](https://en.wikipedia.org/wiki/PageRank#Iterative).

## Code

The project contains two modules. The ```network``` module defines a directed graph to represent the citation network.
You can construct your own network:

```rust
fn main() {
    let mut graph = CitationNetwork::new();
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(0, 3);
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
}
```

or load a network from the data file:

```rust
fn main () {
    let file = File::open("data/cit-HepTh.txt").unwrap();
    let network = CitationNetwork::load_from_file(file);
}
```

To calculate the centrality measures, use ```calculate_degree_centrality``` or 
```calculate_pagerank_centrality```. Both function take a reference to a ```CitationNetwork``` as an argument and
return an instance of ```CentralityRank```, a wrapper for a vector of ```Centrality``` objects for each node, sorted
by their centrality scores in non-ascending order. To obtain the articles with the highest centrality scores, use the
```top(n)``` method of the ```CentralityRank``` class, which returns a subset of ```CentralityRank``` with the highest 
ranking nodes. For example, the main function:

```rust
fn main() {
    let file = File::open("data/cit-HepTh.txt").unwrap();
    let network = CitationNetwork::load_from_file(file);
    let degree_ranks = calculate_degree_centrality(&network);
    println!("Degree Centrality Scores: \n{}", degree_ranks.top(5));
    let pagerank_ranks = calculate_pagerank_centrality(&network);
    println!("PageRank Centrality Scores: \n{}", pagerank_ranks.top(5));
}
```

executed by ```cargo run --release``` should produce the following output:

```
Degree Centrality Scores: 
vertex 9711200: in-degree 2414
vertex 9802150: in-degree 1775
vertex 9802109: in-degree 1641
vertex 9407087: in-degree 1299
vertex 9610043: in-degree 1199

PageRank Centrality Scores: 
vertex 9207016: PageRank 0.006229131514813844
vertex 9407087: PageRank 0.006084355215189933
vertex 9201015: PageRank 0.005638289526055317
vertex 9503124: PageRank 0.004469464403722443
vertex 9510017: PageRank 0.004209784836326371
```

## Result and Analysis

We can see from the output that ranking based on degree centrality does not agree with the ranking based on PageRank.
The first two digits of the node ID represents the year in which the paper was published. For example, article 9207016
was published in 1992, whereas node 9711200 was published in 1997. We can observe that the articles with the highest
degree centrality scores are in general published later than the articles with the highest PageRank scores. This
indicates that as the number of papers published in each year grows, the more recent articles receive more citations,
but the old studies that set the foundation for the field has far-reaching impact that should not be overlooked.
