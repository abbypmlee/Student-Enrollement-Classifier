// This module reads a CSV file into a format suitable for a decision tree, returning organized feature names and instances from the CSV

use crate::tree::Student;
use csv::ReaderBuilder;
use std::fs::File;

// INPUTS: the path to the CSV file as a string slice
// OUTPUTS: a tuple structured like so: (vector of the names of the features as Strings, a vector of instances of the Student struct, one per row of the CSV)
// HIGH-LEVEL LOGIC: Opens the file, extracts feature names from the headers, iterates through the remaining rows, encoding categorical labels and converting everything to f64
                  // For each row, constructs the Student instance and pushes onto a vector to be returned in the tuple
pub fn read_csv(path: &str) -> (Vec<String>, Vec<Student>) {

    // Open the CSV using a ReaderBuilder object
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_reader(File::open(path).expect("Could not open file"));

    // Collect headers as Strings
    let headers = reader
        .headers()
        .expect("Error reading headers")
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // Collect headers into a vector, excluding the target
    let feature_names = headers[..headers.len() - 1].to_vec();
    let mut vec_of_students = Vec::new();

    // Iterate row by row, mapping everything to an f64
    for result in reader.records() {
        let record = result.expect("error reading record");

        let full_row: Vec<f64> = record
            .iter()
            .map(|s| {
                match s {
                    "Dropout" => 1.0,
                    "Enrolled" => 2.0,
                    "Graduate" => 3.0,
                    _ => s.parse::<f64>().expect("not a valid number"), // If anything else, parse as a f64
                }
            })
            .collect(); // Collect into one vector 

        let label = full_row[full_row.len() - 1]; // Label is the last column of each row

        let features = full_row[..full_row.len() - 1].to_vec(); // Features are all the proceeding columns

        let student = Student { features, label };  // Create the instance
        vec_of_students.push(student);  
    }
    (feature_names, vec_of_students)
}
