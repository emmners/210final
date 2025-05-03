use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug,Deserialize)]
pub struct ArrestRecord {
    #[serde(rename = "ARREST_PRECINCT")]
    arrest_precinct: u32,
    #[serde(rename = "OFNS_DESC")]
    ofns_desc: String,
}

pub struct Graph {
    precinct_frequency: HashMap<u32,HashMap<String,u32>>,
    edge_weights: HashMap<(u32,u32),u32>,
}

impl Graph {
    pub fn new() -> Self{
        Self {
            precinct_frequency: HashMap::new(),
            edge_weights: HashMap::new(),
        }
    }
    fn add_row(&mut self, precinct:u32, offense:String) {
        let offense = offense.trim().to_uppercase();
        let precinct_entry = self.precinct_frequency.entry(precinct).or_default();
        let count = precinct_entry.entry(offense).or_insert(0);
        *count += 1;
    }

    fn build_graph(&mut self) {
        let precincts: Vec<u32> self.precinct_frequency.keys().cloned.collect();
        for i in 0..precincts.len() {
            for j in i + 1..precincts.len() {
                let precinct1 = precincts[i];
                let precinct2 = precincts[j];
                let precinct1map = &self.precinct_frequency[&precinct1];
                let precinct2map = &self.precinct_frequency[&precinct2];
                
                let sharedoffenses = Hash
            }
        }
    }
}

pub fn read_csv(path: &str) -> Result<Vec<ArrestRecord>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: ArrestRecord = result?;
        if !record.ofns_desc.trim().is_empty() {
            records.push(record);
        }
    }

    Ok(records)
}