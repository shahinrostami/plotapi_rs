use std::fs;
use std::io::prelude::*;
use nanoid::nanoid;

pub trait Plot {
    fn show(&self);
    fn to_html(&self);
}

#[derive(Debug)]
pub struct Chord {
    pub matrix: Vec<Vec<f64>>,
    pub names: Vec<String>,
    pub colors: String,
    pub opacity: f64,
    pub padding: f64,
    pub width: f64,
    pub label_color: String,
    pub wrap_labels: bool,
    pub margin: f64,
    pub credit: bool,
    pub font_size: String,
    pub font_size_large: String,
}

impl Plot for Chord {
    fn show(&self) {
        let template_url = "https://shahinrostami.com/assets/chord/chord_0_0_12.tmpl";
        let mut res = ureq::get(template_url).call().into_string().unwrap();
        res = res.replace("${tag_id}", &format!("chart-{}", nanoid!()));
        res = res.replace("${matrix}", &format!("{:?}", self.matrix));
        res = res.replace("${names}", &format!("{:?}", self.names));
        res = res.replace("${colors}", &self.colors);
        res = res.replace("${opacity}", &self.opacity.to_string());
        res = res.replace("${padding}", &self.padding.to_string());
        res = res.replace("${width}", &self.width.to_string());
        res = res.replace("${label_color}", &self.label_color);
        res = res.replace("${wrap_labels}", &self.wrap_labels.to_string());
        res = res.replace("${margin}", &self.margin.to_string());
        res = res.replace("${credit}", &self.credit.to_string());
        res = res.replace("${font_size}", &self.font_size);
        res = res.replace("${font_size_large}", &self.font_size_large);

        println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", res);
    }

    fn to_html(&self) {
        let template_url = "https://shahinrostami.com/assets/chord/chord.tmpl";
        let mut res = ureq::get(template_url).call().into_string().unwrap();
        res = res.replace("${tag_id}", &format!("chart-{}", nanoid!()));
        res = res.replace("${matrix}", &format!("{:?}", self.matrix));
        res = res.replace("${names}", &format!("{:?}", self.names));
        res = res.replace("${colors}", &self.colors);
        res = res.replace("${opacity}", &self.opacity.to_string());
        res = res.replace("${padding}", &self.padding.to_string());
        res = res.replace("${width}", &self.width.to_string());
        res = res.replace("${label_color}", &self.label_color);
        res = res.replace("${wrap_labels}", &self.wrap_labels.to_string());
        res = res.replace("${margin}", &self.margin.to_string());
        res = res.replace("${credit}", &self.credit.to_string());
        res = res.replace("${font_size}", &self.font_size);
        res = res.replace("${font_size_large}", &self.font_size_large);
        let file_name = "out.html";

        let mut file = fs::File::create(file_name).unwrap();
        file.write_all(res.as_bytes());
    }
}

impl Default for Chord {
    fn default() -> Self {
        Chord {
            matrix: vec![
                vec![0., 5., 6., 4., 7., 4.],
                vec![5., 0., 5., 4., 6., 5.],
                vec![6., 5., 0., 4., 5., 5.],
                vec![4., 4., 4., 0., 5., 5.],
                vec![7., 6., 5., 5., 0., 4.],
                vec![4., 5., 5., 5., 4., 0.],
            ],
            names: vec![
                "Action",
                "Adventure",
                "Comedy",
                "Drama",
                "Fantasy",
                "Thriller",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            colors: "d3.schemeSet1".to_string(),
            opacity: 0.8,
            padding: 0.01,
            width: 700.0,
            label_color: String::from("#454545"),
            wrap_labels: false,
            margin: 0.0,
            credit: true,
            font_size: String::from("16px"),
            font_size_large: String::from("20px"),
        }
    }
}
