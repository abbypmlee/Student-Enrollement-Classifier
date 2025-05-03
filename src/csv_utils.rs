use crate::tree::Student;
use csv::ReaderBuilder;
use std::fs::File;

pub fn read_csv(path: &str) -> (Vec<String>, Vec<Student>) {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_reader(File::open(path).expect("Could not open file"));

    let headers = reader
        .headers()
        .expect("Error reading headers")
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut vec_of_students = Vec::new();
    let feature_names = headers[..headers.len() - 1].to_vec();

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
            .collect(); // Collect into one vector per row

        let label = full_row[full_row.len() - 1];

        let features = full_row[..full_row.len() - 1].to_vec();

        let student = Student { features, label };
        vec_of_students.push(student);
    }
    (feature_names, vec_of_students)
}
