use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::tree::Student;

pub fn read_csv(path: &str) -> (Vec<String>, Vec<Student>) {        //should return a vector of Students
    let mut answer = Vec::new();     
    let mut feature_names: Vec<String> = Vec::new();         
    let file = File::open(path).expect("Could not open file");         
    let buf_reader = BufReader::new(file).lines();   

    let mut first_row = true;

    for line in buf_reader.into_iter() {
        let line_str = line.expect("Error reading");

        if first_row {
            let all_features: Vec<String> = line_str
                                                .split(";").map(|s| s.to_string()).collect();
            feature_names = all_features[..all_features.len()-1].to_vec();
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
