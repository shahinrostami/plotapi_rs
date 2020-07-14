use std::fs;
use std::io::prelude::*;
use nanoid::nanoid;
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
    


}

impl Plot for Chord {
    fn show(&self) {
        let template_url = "https://shahinrostami.com/assets/chord/chord_0_0_12.tmpl";
        let mut res = ureq::get(template_url).call().into_string().unwrap();
        res = res.replace("${tag_id}", &format!("chart-{}", nanoid!()));
        res = res.replace("${matrix}", &format!("{:?}", self.matrix));
        res = res.replace("${names}", &format!("{:?}", self.names));

        if(self.colors.len() == 1){
            res = res.replace("${colors}",&self.colors[0]);
        }
        else{
            res = res.replace("${colors}", &format!("{:?}", self.colors));
        }

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
        if(self.user.is_empty() && self.key.is_empty()){
            let template_url = "https://shahinrostami.com/assets/chord/chord_0_0_12.tmpl";
            let mut res = ureq::get(template_url).call().into_string().unwrap();
            res = res.replace("${tag_id}", &format!("chart-{}", nanoid!()));
            res = res.replace("${matrix}", &format!("{:?}", self.matrix));
            res = res.replace("${names}", &format!("{:?}", self.names));

            if(self.colors.len() == 1){
                res = res.replace("${colors}",&self.colors[0]);
            }
            else{
                res = res.replace("${colors}", &format!("{:?}", self.colors));
            }
            
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
        else{
            let template_url = "https://api.shahin.dev/chord";
            
            let mut res = ureq::post(template_url)
            .auth(&self.user.to_string(),&self.key.to_string())
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
                "instances":self.instances
            }))
            .into_string().unwrap();

            let file_name = "out.html";
            
            let mut file = fs::File::create(file_name).unwrap();
            file.write_all(res.as_bytes());
        }
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
            wrap_labels: false,
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
        }
    }
}
