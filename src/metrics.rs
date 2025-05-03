// This module prints a variety of metrics for evaluating the decision tree

use linfa::prelude::*;
use linfa_trees::DecisionTree;
use ndarray::{Array1, Array2};

// INPUTS: decision_tree: the decision tree, train: the training split, and train: the training split
// OUTPUTS: (), prints the accuracy
// HIGH-LEVEL LOGIC: initializes the accuracy for the training and testing data, prints them
pub fn print_accuracy(
    decision_tree: DecisionTree<f64, usize>,
    train: &DatasetBase<Array2<f64>, Array1<usize>>,
    test: &DatasetBase<Array2<f64>, Array1<usize>>,
) {
    // Predict on the training data, extract the accuracy from the confusion matrix
    let train_accuracy = decision_tree
        .predict(train)
        .confusion_matrix(train)
        .unwrap()
        .accuracy();

    // Predict on the testing data, extract the accuracy from the confusion matrix
    let test_accuracy = decision_tree
        .predict(test)
        .confusion_matrix(test)
        .unwrap()
        .accuracy();

    // Print both as a percentage
    println!("Train accuracy: {:.2}%", train_accuracy * 100.0);
    println!("Test accuracy: {:.2}%", test_accuracy * 100.0);
}

// INPUTS: decision_tree: the decision tree and train: the training split
// OUTPUTS: (), prints the confusion matrix
// HIGH-LEVEL LOGIC: references the values as usize back to the string slices that were in the CSV, prints the confusion matrix iteratively
pub fn print_confusion_matrix(
    decision_tree: &DecisionTree<f64, usize>,
    test: &DatasetBase<Array2<f64>, Array1<usize>>,
) {
    let class_labels = ["Dropout", "Enrolled", "Graduate"];
    let class_values = [1, 2, 3];

    println!("\nConfusion Matrix:");
    print!("{:<16}", "Actual \\ Pred");
    // Pring the labels for the columns by iterating through the labels
    for (i, _pred) in class_values.iter().enumerate() {
        print!("{:<16}", class_labels[i]);
    }
    println!();

    let predictions = decision_tree.predict(test);
    let actual = test.targets();

    // Iterate through the rows of the confusion matrix using class_values
    for (j, class_row) in class_values.iter().enumerate() {
        // Print each label at the beginning of each row
        print!("{:<16}", class_labels[j]);
        // Iterate through the columns of the confusion matrix using class_values
        for &class_column in &class_values {
            // For each rowxcolumn combination, initalize count to 0
            let mut count = 0;
            for (pred, actual_value) in predictions.iter().zip(actual.iter()) {
                if *actual_value == *class_row && *pred == class_column {
                    // If actual == row and predicted == column, increment by 1
                    count += 1;
                }
            }
            // Print the  count for each combination of actual and predicted labels
            print!("{:<16}", count);
        }
        println!();
    }
}

// INPUTS: decision_tree: the decision tree and train: the training split
// OUTPUTS: (), prints the first ten predictions
// HIGH-LEVEL LOGIC: iterates through ten Students and compares the predicted label with the actual one
pub fn print_first_ten_predictions(
    decision_tree: &DecisionTree<f64, usize>,
    test: &DatasetBase<Array2<f64>, Array1<usize>>,
) {
    let predictions = decision_tree.predict(test);
    let actual = test.targets();

    println!("First 10 Predictions vs Actual Labels:");
    for i in 0..10 {
        let pred = predictions[i];      // Compare predictions for that index
        let true_label = actual[i];     // With the actual value
        println!(
            "Sample {}: Predicted = {}, Actual = {}",
            i + 1,
            pred,
            true_label
        );
    }
}
