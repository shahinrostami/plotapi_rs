mod lib;
use lib::{Chord, Plot};
use std::fs;
use std::io::prelude::*;

fn main() {
    let matrix: Vec<Vec<f64>> = vec![
    vec![0.0, 0.0, 0.0, 1.0, 4.0, 1.0],
    vec![0.0, 0.0, 0.0, 1.0, 3.0, 2.0],
    vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0],
    vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0],
    vec![4.0, 3.0, 2.0, 0.0, 0.0, 0.0],
    vec![1.0, 2.0, 2.0, 0.0, 0.0, 0.0],
    ];

    let names: Vec<String> = vec!["A", "B", "C", "1", "2", "3"]
    .into_iter()
    .map(String::from)
    .collect();

    let colors: Vec<String> = vec!["#7400B8", "#5E60CE", "#5684D6", "#56CFE1", "#64DFDF", "#80FFDB"]
    .into_iter()
    .map(String::from)
    .collect();
    

    lib::Chord {
      user: String::from("hello@shahinrostami.com"),
        key: String::from("CP-2233d274-f968-4018-870b-4926b1793912"),
        matrix: matrix.clone(),
        names: names.clone(),
       colors: colors,
        divide: true,
        divide_idx: 3,
        ..lib::Chord::default()
    }
    .to_html();

}
