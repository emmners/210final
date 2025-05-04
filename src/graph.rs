use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::collections::VecDeque;

//this module is to build the graph and cluster the precincts based on whether it meets the threshold of the frequency of overlapping crimes. 
#[test]
fn test_add_single_row() {
    let mut graph = Graph::new();
    graph.add_row(101, "THEFT".to_string());
    
    assert_eq!(graph.precinct_frequency[&101]["THEFT"], 1);
}

#[test]
fn frequency() { 
    let mut graph = Graph::new();
    graph.add_row(101, "THEFT".to_string());
    graph.add_row(101, "THEFT".to_string());
    graph.add_row(2,"THEFT".to_string());
    graph.build_graph();
    assert_eq!(graph.edge_weights.get(&(101,2)),SomSe(&1));
}

#[derive(Debug,Deserialize)]
pub struct ArrestRecord { //ArrestRecord is used to eliminate unnecessary columns and make it easy to grab the values from the csv immediately and add it to the graph right away in csv.rs
    #[serde(rename = "ARREST_PRECINCT")]
    pub arrest_precinct: u32,
    #[serde(rename = "PD_DESC")]
    pub pd_desc: String,
}

pub struct Graph {
    precinct_frequency: HashMap<u32,HashMap<String,u32>>,
    edge_weights: HashMap<(u32,u32),u32>,
}

impl Graph {
    //This creates a new graph that can be used by the other functions being implemented. 
    pub fn new() -> Self{
        Self {
            precinct_frequency: HashMap::new(),
            edge_weights: HashMap::new(),
        }
    }

    //This lets new rows be added to the empty graph. For each row, it is the precinct number, and a hashmap that holds the offense description and the frequency of arrests for that offense
    pub fn add_row(&mut self, precinct:u32, offenses:String) { //we take offenses as a string so for every arrest in the csv, we can just grab the precinct # and pd_desc, adding each row as it reads the csv
        let offenses = offenses.trim().to_uppercase();
        let precinct_entry = self.precinct_frequency.entry(precinct).or_default();
        let count = precinct_entry.entry(offenses).or_insert(0);
        *count += 1;
    }

    //this function builds the graph by finding shared offenses between each combination of precincts, and then finding the weights by adding the minimum of the frequency per offense. 
    pub fn build_graph(&mut self) {
        let precincts: Vec<u32> = self.precinct_frequency.keys().cloned().collect(); //this is a vector of all the precincts so I can iterate through them
        for i in 0..precincts.len() {
            for j in i + 1..precincts.len() { //going through all the combinations of precincts
                let precinct1 = precincts[i];
                let precinct2 = precincts[j];
                let precinct1map = &self.precinct_frequency[&precinct1]; //access frequencies for offenses per precincts
                let precinct2map = &self.precinct_frequency[&precinct2];
                
                let precinct1keys = precinct1map.keys().collect::<HashSet<_>>(); //
                let precinct2keys = precinct2map.keys().collect::<HashSet<_>>();
                let sharedoffenses: HashSet<&String> = precinct1keys.intersection(&precinct2keys).cloned().collect(); //find the intersections between offense descriptions

                let mut weight: u32 = 0;
                for offense in &sharedoffenses { //for each offense in the shared offenses, edge weights between precincts = sum of totals of the minimum amount that they share 
                    let precinct1count = precinct1map.get(offense.as_str()).copied().unwrap_or(0);
                    let precinct2count = precinct2map.get(offense.as_str()).copied().unwrap_or(0);

                    weight += precinct1count.min(precinct2count);

                }
                
                if weight > 0 {
                    self.edge_weights.insert((precinct1,precinct2),weight);
                } 
            }
        }
    }

    pub fn print_weighted_edges(&self) { //print the edges to check numbers!
        println!("Weighted Precinct Graph Edges:");
        // println!("{:?}",(&self.edge_weights).len());
        let mut weighttotal = 0;
        for ((precinct1, precinct2), weight) in &self.edge_weights { 
            println!("Precinct {} ↔ {}  → Weight: {}", precinct1, precinct2, weight);
            weighttotal += weight;
        }
        println!("average edge weight:{:?}",weighttotal / (&self.edge_weights).len() as u32);
    }
    pub fn dfs(&mut self,threshold:u32) -> Vec<HashSet<u32>> {
        let mut visited = HashSet::new();  //tracks precincts that have been looked at 
        let mut clusters: Vec<HashSet<u32>> = Vec::new(); //holds final list of clusters
    
        let mut adjacencylist: HashMap<u32, Vec<u32>> = HashMap::new();  //builds undirected adjacency list, only creates connections if weight is above set threshold
        for ((precinct1, precinct2), weight) in &self.edge_weights { 
            if weight >= &threshold {
                adjacencylist.entry(*precinct1).or_default().push(*precinct2);
                adjacencylist.entry(*precinct2).or_default().push(*precinct1);
            }
        }
    
        for &precinct in self.precinct_frequency.keys() { //dfs algorithm, goes through all precincts
            if !visited.contains(&precinct) { //if the precinct hasn't been through dfs
                let mut cluster: HashSet<u32> = HashSet::new(); 
                let mut stack = VecDeque::new();
                stack.push_back(precinct); //creates a new cluster and adds the precinct to the stack
    
                while let Some(currentnode) = stack.pop_back() { //if there's a precinct in the stack, it will look at it 
                    if visited.insert(currentnode) { //adds current precinct to visted hashset if it hasn't been visited
                        cluster.insert(currentnode); //adds precinct to cluster
                        
                        if let Some(neighbors) = adjacencylist.get(&currentnode) { //checks ajacency list for neighbors for the next step 
                            for neighbor in neighbors {
                                if !visited.contains(neighbor) {
                                    stack.push_back(*neighbor);
                                } //keeps looping through until stack is empty and there are no more neighbors/connections
                            }
                        }
                    }
                }
                if !cluster.is_empty() {
                    clusters.push(cluster); //if the cluster's not empty, push the cluster into the final clusters variable
                }
            }
        }
        clusters
    }
    pub fn print_clusters(&mut self) { //prints the final clusters
        //took the average edge weight and * 0.75
        let clusters = self.dfs(1350); //runs through dfs using a set threshold(75% of average edge weight(1795))
        for (i,cluster) in clusters.iter().enumerate() { 
            let nodes: Vec<_> = cluster.iter().cloned().collect();
            println!("Cluster {:?}: {:?}",i + 1,nodes); //collects each cluster and prints the precincts of each out one by one
        }
    }
}