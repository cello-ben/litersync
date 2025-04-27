use crate::consts;

use serde_json;

fn url_encode(title: &str) -> String {
    //We'll just deal with spaces for now. TODO more robust solution.
    return title.replace(" ", "%20");
}

pub fn get_json(title: &str) -> String {
    let query_url: String = vec![&consts::API_BASE_URL, "?title=", &url_encode(title), "&format=json"].join("");
    match reqwest::blocking::get(query_url.clone()) {
        Ok(response) => match response.json::<serde_json::Value>() {
            Ok(json) => println!("{:?}", json),
            Err(e) => println!("{}", e)
        }
        Err(e) => println!("{}", e)
    }
    query_url
}