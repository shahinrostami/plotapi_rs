use std::fs;
use std::io::prelude::*;
use ureq::SerdeValue;

pub use ureq::json as params;

pub struct Visualisation {
    pub api_key: &'static str,
    pub endpoint: &'static str,
    pub params: SerdeValue,
}

impl Visualisation {
    fn endpoint_url(&self) -> String {
        ["https://plotapi.com/", &self.endpoint, "/"].concat()
    }

    pub fn show(&self) {
        let html = self.render();
        println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", html);
    }

    pub fn to_html(&self) {
        let html = self.render();
        let file_name = "out.html";

        let mut file = fs::File::create(file_name).unwrap();
        file.write_all(html.as_bytes())
            .expect("writing to output file failed");
    }

    fn render(&self) -> String {
        ureq::post(&self.endpoint_url())
            .auth("api_key", &self.api_key.to_string())
            .send_json(self.params.clone())
            .into_string()
            .unwrap()
    }
}
