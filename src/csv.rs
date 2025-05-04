use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use crate::graph::ArrestRecord;
use crate::graph::Graph;

//this module is to read the csv and turn it into a graph using the graph functions in graph.rs.

pub fn read_csv(path: &str) -> Result<Graph, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut graph = Graph::new();

    for result in rdr.deserialize() {
        let record: ArrestRecord = result?;
        if !record.pd_desc.trim().is_empty() {
            graph.add_row(record.arrest_precinct,record.pd_desc);
        } //for each row in the csv, create an arrestrecord that has the arrest precinct and pd_description, and add the row to a new graph
    }

    Ok(graph) 
}