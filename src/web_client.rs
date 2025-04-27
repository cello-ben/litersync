use crate::consts;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct URLWrapper {
    url_zip_file: String
}

fn url_encode(title: &str) -> String {
    //We'll just deal with spaces for now. TODO more robust solution.
    return title.replace(" ", "%20");
}

pub fn get_json(title: &str) -> String {
    let mut query_url: String = consts::API_BASE_URL.to_owned();
    query_url.push_str("?title=");
    query_url.push_str(&url_encode(title));
    query_url.push_str("&format=json");
    match reqwest::blocking::get(query_url.clone()) {
        Ok(response) => println!("{:?}", response.json().unwrap()),
        Err(e) => println!("{}", e)
    }
    query_url
}