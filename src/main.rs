mod csv_utils;
mod metrics;
mod tree;

use csv_utils::read_csv;
use metrics::print_metrics;
use tree::{build_tree, tree_prediction};

fn main() {
    let (feature_names, v) = read_csv("src/college_data.csv"); //  tuple, (feature_names, Vec<Student>)
    let (decision_tree, train, test) = build_tree(feature_names, v.clone());

    println!(
        "the predicted class is {:?}",
        tree_prediction(decision_tree.clone(), v[0].features.clone())
    );

    print_metrics(decision_tree.clone(), train, test);
}
