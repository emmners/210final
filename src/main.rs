mod graph;
mod csv;
use crate::csv::read_csv;
use crate::graph::Graph;

#[test] 
fn test_disconnected_nodes() {
    let mut graph = Graph::new();
    graph.add_row(1, "THEFT".to_string());
    graph.add_row(2, "ROBBERY".to_string());

    graph.build_graph();
    let clusters = graph.dfs(1);

     assert_eq!(clusters.len(), 2);
}

fn main() { //read csv, build the graph, print edges, and print final clusters
    let path = "updated_NYPD_Arrest_Data_YTD.csv";
    let mut graph = read_csv(path).expect("reading failure");
    graph.build_graph();
    graph.print_weighted_edges();
    graph.print_clusters();
}
