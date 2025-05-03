// This module constructs and trains a decision tree based off of feature names and a vector of samples; it also has a function to predict an arbitrary student's label.

use linfa::prelude::*;
use linfa_trees::DecisionTree;
use ndarray::{Array1, Array2};

// The Student struct represents one instance from the sample
// It contains the features as a vector of f64 and the label as a single f64, after being encoded numerically from read_csv()
// It is used to represent one individual in the CSV
#[derive(Clone, Debug)]
pub struct Student {
    pub features: Vec<f64>,
    pub label: f64,
}

// INPUTS: feature_names: A vector of feature names (Vec<String>) and v: A vector of Student instances, each containing features and a label
// OUTPUTS: a decision tree from linfa_trees as well as train and test splits (DatasetBases from linfa)
// HIGH-LEVEL LOGIC: Flattens student feature data into a shape suitable for ndarray, creates an array for labels, creates a dataset and runs an 80/20 split, returns the tree and the split 
pub fn build_tree(
    feature_names: Vec<String>,
    v: Vec<Student>,
    max_depth: usize,
) -> (
    DecisionTree<f64, usize>,
    DatasetBase<Array2<f64>, Array1<usize>>,
    DatasetBase<Array2<f64>, Array1<usize>>,
) {
    let mut flat_values: Vec<f64> = Vec::new(); // Stores all feature values
    let mut label_vec: Vec<usize> = Vec::new(); // Stores all class labels

    for student in &v {
        for feature in &student.features {
            flat_values.push(*feature);
        }
        label_vec.push(student.label as usize); // Pushing on all the features 
    }

    // Creating an array, # rows = # Students, # columns = # features
    let array = Array2::from_shape_vec((v.len(), feature_names.len()), flat_values)
        .expect("Error creating ndarray");  

    let labels: Array1<usize> = Array1::from_vec(label_vec);   

    let dataset = Dataset::new(array, labels).with_feature_names(feature_names);

    let (train, test) = dataset.split_with_ratio(0.8); // 80% train, 20% test

    let decision_tree = DecisionTree::params()  // Create tree
        .max_depth(Some(max_depth))
        .fit(&train)
        .unwrap();

    (decision_tree, train, test)    // Return tree, train, test split
}

// INPUTS: decision_tree: a decision tree built from the build_tree function and features: a vector of features
// OUTPUTS: a String representing the predicted label of the student
// HIGH-LEVEL LOGIC: converts the features to an array and makes a prediction before mapping the prediction (a usize) back to &str -> String
pub fn predict_one_student(decision_tree: DecisionTree<f64, usize>, features: Vec<f64>) -> String {
    let input = Array2::from_shape_vec((1, features.len()), features)
        .expect("Failed to reshape features into array");

    // prediction
    let prediction = decision_tree.predict(&input);

    // match prediction (usize) back to a &str
    let answer = match prediction[0] {
        1 => "Dropout",
        2 => "Enrolled",
        3 => "Graduate",
        _ => "Invalid prediction",
    };

    // print the prediction
    println!("The predicted label for this student is: {:?}", answer);

    // convert back to a String and return
    answer.to_string()
}
