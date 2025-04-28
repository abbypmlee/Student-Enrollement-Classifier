use std::fs::File;   
use std::io::prelude::*;
use std::collections::HashMap;

pub type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;
type ListOfWeights = Vec<usize>;
pub struct Graph {
    pub n: usize,  
    pub outedges: AdjacencyLists,
    pub total_edges: ListOfEdges,
    pub weights: ListOfWeights,
}

fn read_csv_to_hashmap(path: &str) -> HashMap<usize, Vec<f64>> {
    let mut answer = HashMap::new();              
    let file = File::open(path).expect("Could not open file");         
    let buf_reader = std::io::BufReader::new(file).lines();   

    let mut first_row = true;

    for (index, line) in buf_reader.enumerate() {
        let line_str = line.expect("Error reading");

        if first_row {
            first_row = false;
            continue;  
        }
    
        let features: Vec<f64> = line_str
                                                .split(";")
                                                .map(|s| {
                                                    match s { 
                                                        "Dropout" => 1.0,
                                                        "Enrolled" => 2.0,
                                                        "Graduate" => 3.0,
                                                        _ => s.parse::<f64>().expect("not a valid number"),
                                                    }
                                                }
                                                    )
                                                .collect();

        answer.insert(index, features);
            }
    answer
    }


fn main() {
    read_csv_to_hashmap("college_data.csv");
}


//todo: read_csv
//create graph
//operations from there



//FOR OFFICE HOURS
//git? project idea? how to do visualization?
