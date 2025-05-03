// Contains the entry to my program and my test moduel

// Import all modules
mod csv_reading;
mod metrics;
mod tree;

// Use necessary functions
use rand::Rng;
use csv_reading::read_csv;
use metrics::{print_accuracy, print_confusion_matrix, print_first_ten_predictions};
use tree::{build_tree, predict_one_student};

fn main() {

    let depth = 6;      // Set arbitrary depth; here I set to 6
    let (feature_names, v) = read_csv("src/college_data.csv");      // Read the CSV
    let (decision_tree, train, test) = build_tree(feature_names, v.clone(), depth);     // construct the decision tree

    // Generate a random number for the arbitrary student whose label will be predicted
    let mut rng = rand::thread_rng();  
    let num = rng.gen_range(0..v.len());

    println!("");
    println!("FOR A TREE WITH A DEPTH OF {}:", depth);
    println!("");
    println!("For student {}:", num);
    predict_one_student(decision_tree.clone(), v[num].features.clone());        // Predict the label for that student
    println!("");
    println!(
        "the actual label for that student is {:?}",        // Print the actual label for that student
        match v[num].label {
            1.0 => "Dropout",
            2.0 => "Enrolled",
            3.0 => "Graduate",
            _ => "Unknown",
        }
    );
    println!("");
    print_accuracy(decision_tree.clone(), &train, &test);       // Print the accuracy of the decision tree on the training and testing data  
    print_confusion_matrix(&decision_tree, &test);      // Print the confusion matrix for the tree that was constructed
    println!("");
    print_first_ten_predictions(&decision_tree, &test); // Print the first ten predictions using the tree
}

// This is my test module; it contains four tests, two for csv_reading.rs, one for tree.rs, and one for metrics.rs
#[cfg(test)]
mod tests {
    use super::*;

    // This test makes sure that read_csv is identifying the correct label for a given student
    #[test]
    fn test_read_csv_one_row() {
        let path = "src/college_data.csv";
        let (_feature_names, v) = read_csv(path);
        assert_eq!(v[5].label, 3.0);
    }

    // This test makes sure that all students are being pushed onto the returned vector in read_csv
    #[test]
    fn test_student_vec_length() {
        let path = "src/college_data.csv";
        let (_vec_features, vec_students) = read_csv(path);
        assert_eq!(vec_students.len(), 4424);
    }

    // This test makes sure that predict_one_student() returns a value that is valid from the CSV
    #[test]
    fn test_predict_one_student() {
        let path = "src/college_data.csv";
        let (feature_names, vec_of_students) = read_csv(path);
        let (decision_tree, _train, _test) = build_tree(feature_names, vec_of_students.clone(), 6);
        assert!(
            (predict_one_student(decision_tree.clone(), vec_of_students[5].features.clone())
                == "Dropout")
                || (predict_one_student(
                    decision_tree.clone(),
                    vec_of_students[5].features.clone()
                ) == "Enrolled")
                || (predict_one_student(
                    decision_tree.clone(),
                    vec_of_students[5].features.clone()
                ) == "Graduate")
        );
    }

    // This test makes sure that everything needed to print the confusion matrix runs without crashing
    #[test]
    fn test_print_confusion_matrix_runs() {
        let path = "src/college_data.csv";
        let (features, students) = read_csv(path);
        let (tree, _, test) = build_tree(features, students, 3);
        print_confusion_matrix(&tree, &test);
    }
}
