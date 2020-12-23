mod lib;
use lib::{Chord, Plot};
use std::fs;
use std::io::prelude::*;

fn main() {
    let matrix: Vec<Vec<f64>> = vec![
    vec![0., 5., 6., 4., 7., 4.],
    vec![5., 0., 5., 4., 6., 5.],
    vec![6., 5., 0., 4., 5., 5.],
    vec![4., 4., 4., 0., 5., 5.],
    vec![7., 6., 5., 5., 0., 4.],
    vec![4., 5., 5., 5., 4., 0.],
    ];

    let details : Vec<Vec<Vec<String>>> = vec![
        vec![vec![], vec!["Movie 1".to_string(),"Movie 2".to_string()], vec!["Movie 3".to_string(),"Movie 4".to_string(),"Movie 5".to_string()], vec!["Movie 6".to_string(),"Movie 7".to_string()], vec!["Movie 8".to_string(),"Movie 9".to_string(),"Movie 10".to_string(),"Movie 11".to_string()], vec!["Movie 12".to_string()]],
        vec![vec!["Movie 13".to_string(),"Movie 14".to_string()], vec![], vec!["Movie 15".to_string(),"Movie 16".to_string()], vec!["Movie 17".to_string()], vec!["Movie 18".to_string(),"Movie 19".to_string(),"Movie 20".to_string()], vec!["Movie 21".to_string(),"Movie 22".to_string()]],
        vec![vec!["Movie 23".to_string(),"Movie 24".to_string(),"Movie 25".to_string()], vec!["Movie 26".to_string(),"Movie 27".to_string()], vec![], vec!["Movie 28".to_string()], vec!["Movie 29".to_string(),"Movie 30".to_string()], vec!["Movie 31".to_string(),"Movie 32".to_string()]],
        vec![vec!["Movie 33".to_string()], vec!["Movie 34".to_string()], vec!["Movie 35".to_string()], vec![], vec!["Movie 36".to_string(),"Movie 37".to_string()], vec!["Movie 38".to_string(),"Movie 39".to_string()]],
        vec![vec!["Movie 40".to_string(),"Movie 41".to_string(),"Movie 42".to_string(),"Movie 43".to_string()], vec!["Movie 44".to_string(),"Movie 45".to_string(),"Movie 46".to_string()], vec!["Movie 47".to_string(),"Movie 48".to_string()], vec!["Movie 49".to_string(),"Movie 50".to_string()], vec![], vec!["Movie 51".to_string()]],
        vec![vec!["Movie 52".to_string()], vec!["Movie 53".to_string(),"Movie 54".to_string()], vec!["Movie 55".to_string(),"Movie 56".to_string()], vec!["Movie 57".to_string(),"Movie 58".to_string()], vec!["Movie 59".to_string()], vec![]]
    ];

    let details_thumbs : Vec<Vec<Vec<String>>> = vec![
        vec![vec![], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string()]],
        vec![vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec![], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()]],
        vec![vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec![], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()]],
        vec![vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec![], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()]],
        vec![vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec![], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string()]],
        vec![vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string(),"https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec!["https://shahinrostami.com/images/stami-labs/lablet.png".to_string()], vec![]],
    ];

    let names: Vec<String> = vec![
        "Action",
        "Adventure",
        "Comedy",
        "Drama",
        "Fantasy",
        "Thriller",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    lib::Chord {
        user: String::from("enter username here"),
        key: String::from("enter license key here"),
        matrix: matrix.clone(),
        names: names.clone(),
        details: details,
        details_thumbs: details_thumbs,
        ..lib::Chord::default()
    }
    .to_html();
}
