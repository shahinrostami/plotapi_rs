use std::fs;
use std::io::prelude::*;
use ureq::json;

pub trait Plot {
    fn show(&self);
    fn to_html(&self);
}

#[derive(Debug)]
pub struct Chord {
    pub user: String,
    pub key: String,
    pub matrix: Vec<Vec<f64>>,
    pub names: Vec<String>,
    pub colors: Vec<String>,
    pub opacity: f64,
    pub padding: f64,
    pub width: f64,
    pub label_color: String,
    pub wrap_labels: bool,
    pub margin: f64,
    pub credit: bool,
    pub font_size: String,
    pub font_size_large: String,
    pub details: Vec<Vec<Vec<String>>>,
    pub details_thumbs: Vec<Vec<Vec<String>>>,
    pub thumbs_width: f64,
    pub thumbs_margin: f64,
    pub thumbs_font_size: f64,
    pub popup_width: f64,
    pub noun: String,
    pub details_separator: String,
    pub divide: bool,
    pub divide_idx: u64,
    pub divide_size: f64,
    pub instances: u64,
    pub verb: String,
    pub symmetric: bool,
    pub title: String,
    pub arc_numbers: bool,
    pub divide_left_label: String,
    pub divide_right_label: String,
    pub inner_radius_scale: f64,
    pub outer_radius_scale: f64,
    pub allow_download: bool,
    pub conjunction: String,
    pub reverse_gradients: bool,
    pub curved_labels: bool,
}

impl Chord {
    fn endpoint_url(&self) -> &'static str {
        if self.user.is_empty() && self.key.is_empty() {
            "https://api.shahin.dev/chordfree"
        } else {
            "https://api.shahin.dev/chord"
        }
    }

    fn render(&self) -> String {
        ureq::post(self.endpoint_url())
            .auth(&self.user.to_string(), &self.key.to_string())
            .send_json(json!({
                "colors":self.colors,
                "opacity":self.opacity,
                "matrix":self.matrix,
                "names":self.names,
                "padding":self.padding,
                "width":self.width,
                "label_color":self.label_color,
                "wrap_labels":self.wrap_labels,
                "credit":self.credit,
                "margin":self.margin,
                "font_size":self.font_size,
                "font_size_large":self.font_size_large,
                "details":self.details,
                "details_thumbs":self.details_thumbs,
                "thumbs_font_size":self.thumbs_font_size,
                "thumbs_width":self.thumbs_width,
                "thumbs_margin":self.thumbs_margin,
                "popup_width":self.popup_width,
                "noun":self.noun,
                "details_separator":self.details_separator,
                "divide":self.divide,
                "divide_idx":self.divide_idx,
                "divide_size":self.divide_size,
                "instances":self.instances,
                "verb": self.verb,
                "symmetric": self.symmetric,
                "title": self.title,
                "arc_numbers": self.arc_numbers,
                "divide_left_label": self.divide_left_label,
                "divide_right_label": self.divide_right_label,
                "inner_radius_scale": self.inner_radius_scale,
                "outer_radius_scale": self.outer_radius_scale,
                "allow_download": self.allow_download,
                "conjunction": self.conjunction,
                "reverse_gradients": self.reverse_gradients,
                "curved_labels":self.curved_labels
            }))
            .into_string()
            .unwrap()
    }
}

impl Plot for Chord {
    fn show(&self) {
        let html = self.render();
        println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", html);
    }

    fn to_html(&self) {
        let html = self.render();
        let file_name = "out.html";

        let mut file = fs::File::create(file_name).unwrap();
        file.write_all(html.as_bytes())
            .expect("writing to output file failed");
    }
}

impl Default for Chord {
    fn default() -> Self {
        Chord {
            user: String::new(),
            key: String::new(),
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
            colors: vec![String::from("d3.schemeSet1")],
            opacity: 0.8,
            padding: 0.01,
            width: 700.0,
            label_color: String::from("#454545"),
            wrap_labels: true,
            margin: 0.0,
            credit: true,
            font_size: String::from("16px"),
            font_size_large: String::from("20px"),
            details: vec![],
            details_thumbs: vec![],
            thumbs_width: 85.0,
            thumbs_margin: 5.0,
            thumbs_font_size: 14.0,
            popup_width: 350.0,
            noun: String::from("instances"),
            details_separator: String::from(", "),
            divide: false,
            divide_idx: 0,
            divide_size: 0.5,
            instances: 0,
            verb: String::from("occur together in"),
            symmetric: true,
            title: String::from(""),
            arc_numbers: false,
            divide_left_label: String::from(""),
            divide_right_label: String::from(""),
            inner_radius_scale: 0.39,
            outer_radius_scale: 1.1,
            allow_download: false,
            conjunction: String::from("and"),
            reverse_gradients: false,
            curved_labels: false,
        }
    }
}