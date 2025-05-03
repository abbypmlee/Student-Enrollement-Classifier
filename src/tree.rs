use linfa::prelude::*;
use linfa_trees::DecisionTree;
use ndarray::{Array1, Array2};

#[derive(Clone, Debug)]
pub struct Student {
    pub features: Vec<f64>,
    pub label: f64,
}

pub fn build_tree(
    feature_names: Vec<String>,
    v: Vec<Student>,
) -> (
    DecisionTree<f64, usize>,
    DatasetBase<Array2<f64>, Array1<usize>>,
    DatasetBase<Array2<f64>, Array1<usize>>,
) {
    let mut flat_values: Vec<f64> = Vec::new(); // Stores all feature values
    let mut status_vec: Vec<usize> = Vec::new(); // Stores all class labels

    for student in &v {
        for feature in &student.features {
            flat_values.push(*feature);
        }
        status_vec.push(student.label as usize);
    }

    let array = Array2::from_shape_vec((v.len(), feature_names.len()), flat_values)
        .expect("Error creating ndarray");

    let status: Array1<usize> = Array1::from_vec(status_vec);

    let dataset = Dataset::new(array, status).with_feature_names(feature_names);

    let (train, test) = dataset.split_with_ratio(0.8); // 80% train, 20% test

    let decision_tree = DecisionTree::params()
        .max_depth(Some(6))
        .fit(&train)
        .unwrap();

    (decision_tree, train, test)
}

pub fn tree_prediction(decision_tree: DecisionTree<f64, usize>, features: Vec<f64>) -> usize {
    let input = Array2::from_shape_vec((1, features.len()), features)
        .expect("Failed to reshape features into array");
    let prediction = decision_tree.predict(&input);
    prediction[0]
}
