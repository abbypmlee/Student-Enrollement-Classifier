mod csv_utils;
mod tree;

use csv_utils::read_csv;
use tree::{build_tree, tree_prediction};

fn main() {
    let (feature_names, v) = read_csv("src/college_data.csv"); //  tuple, (feature_names, Vec<Student>)
    let (decision_tree, train, test) = build_tree(feature_names, v.clone());

   // let test = v.clone();

   // println!("{:?}", test[0].features);
  // tree_prediction(decision_tree, v[0].features);

    println!("the predicted class is {:?}", tree_prediction(decision_tree, v[0].features.clone()));

}
