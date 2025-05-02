use crate::tree::Student;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_csv(path: &str) -> (Vec<String>, Vec<Student>) {
    let mut vec_of_students = Vec::new();
    let mut feature_names: Vec<String> = Vec::new();

    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file).lines(); //WHY NOT USE READERBUILDER HERE? HASHEADERS, PATH?

    let mut first_row = true;

    for row in buf_reader.into_iter() {
        let row_str = row.expect("Error reading");

        if first_row {
            let all_features: Vec<String> = row_str.split(";").map(|s| s.to_string()).collect();
            feature_names = all_features[..all_features.len() - 1].to_vec();
            first_row = false; // Move onto other rows
            continue; // Treat them differently
        }

        let full_row: Vec<f64> = row_str
            .split(";")
            .map(|s| {
                match s {
                    "Dropout" => 1.0,
                    "Enrolled" => 2.0,
                    "Graduate" => 3.0,
                    _ => s.parse::<f64>().expect("not a valid number"), // If anything else, parse as a f64
                }
            })
            .collect(); // Collect into one vector per row

        let label = full_row[full_row.len() - 1];

        let features = full_row[0..full_row.len() - 1].to_vec();

        let student = Student { features, label };
        vec_of_students.push(student);
    }
    (feature_names, vec_of_students)
}
