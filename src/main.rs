use tree::run_tree;
mod tree;
mod csv_utils;
use csv_utils::read_csv;



fn main() {
    let (feature_names, v) = read_csv("college_data.csv"); //  tuple, (feature_names, Vec<Student>)
    run_tree(feature_names, v);


}



