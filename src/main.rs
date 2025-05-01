use std::fs::File;   
use std::io::prelude::*;
mod tree;

pub struct Student {
    features: Vec<f64>,
    label: f64,
}

pub fn read_csv(path: &str) -> (Vec<String>, Vec<Student>) {        //should return a vector of Students
    let mut answer = Vec::new();     
    let mut feature_names: Vec<String> = Vec::new();         
    let file = File::open(path).expect("Could not open file");         
    let buf_reader = std::io::BufReader::new(file).lines();   

    let mut first_row = true;

    for line in buf_reader.into_iter() {
        let line_str = line.expect("Error reading");

        if first_row {
            feature_names = line_str
                                                .split(";").map(|s| s.to_string()).collect();
            first_row = false;
            continue;  
        }
    
        let full_row: Vec<f64> = line_str
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
        
        let label = full_row[full_row.len()-1];
        let features = full_row[0..full_row.len()-1].to_vec();

        let student = Student {features, label};

        answer.push(student);
            }
    (feature_names, answer)
    }

fn main() {

}

//create graph
//operations from there


//k nearest neighbors to see if there are clusters within the features 
//draw clusters in two dimensions using dimensionality reduction techniques; use ndarray 
//use PCA to reduce dimensionality 
//use a cargo crate for different clusters, everyone gets assigned to a cluster 
// measure the distance of every point to its centroid in its cluster
// the more clusters, the closer every point is going to be to its centroid 
// use k means cluster plot in plotly 
// run PCA to project points into two dimensions, plot that as a scatter plot BEFORE K MEANS, get a sense of if there is a random cloud
// if random cloud, k means might not work that well 
// if clusters, k means will work well 
// it's ok if you don't get results, not a DS project, just run PCA and k means and it'll be fine 


// run a decision tree algorithm to predict k means; k means tells you similarity, decision tree predicts 
// K MEANS OR DECISION TREE

//try decision tree first 

