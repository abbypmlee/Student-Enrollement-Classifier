use ndarray::{Array1, Array2};
use linfa::prelude::*;
use linfa_trees::DecisionTree;

pub struct Student {
    pub features: Vec<f64>,
    pub label: f64,
}

pub fn run_tree(feature_names: Vec<String>, v: Vec<Student>) {
    //let (feature_names, v) = read_csv("college_data.csv"); //  tuple, (feature_names, Vec<Student>)
    let mut flat_values: Vec<f64> = Vec::new();
    let mut status_vec: Vec<f64> = Vec::new();

    for student in &v {
        for feature in &student.features {
            flat_values.push(*feature);
        }
        status_vec.push(student.label);
      }
    let array = Array2::from_shape_vec((v.len(), feature_names.len()), flat_values).expect("Error creating ndarray");
  
    let status: Array1<usize> = Array1::from_vec(status_vec.iter().map(|x| *x as usize).collect());
  
    let dataset = Dataset::new(array, status).with_feature_names(feature_names);


    let (train, test) = dataset.split_with_ratio(0.8); // 80% train, 20% test

    let decision_tree = DecisionTree::params()
        .max_depth(Some(6))
        .fit(&train)
        .unwrap();
    
    let accuracy = decision_tree
        .predict(&test)
        .confusion_matrix(&test)
        .unwrap()
        .accuracy();
    
    println!("Test accuracy: {:.2}%", accuracy * 100.0);



   // let decision_tree = DecisionTree::params()
     //     .max_depth(Some(6))
       //   .fit(&dataset)
       //   .unwrap();
  
    //let accuracy = decision_tree.predict(&dataset).confusion_matrix(&dataset).unwrap().accuracy();
      
   // println!("The accuracy is: {:?}", accuracy);
  
   
    }
  
