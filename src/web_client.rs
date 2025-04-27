// Copyright 2025 Benjamin Fryxell

// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


use crate::consts;

use serde_json;

fn url_encode(title: &str) -> String {
    //We'll just deal with spaces for now. TODO more robust solution.
    return title.replace(" ", "%20");
}

pub fn get_json(title: &str) -> String {
    let query_url: String = vec![&consts::API_BASE_URL, "?title=", &url_encode(title), "&format=json"].join("");
    match reqwest::blocking::get(query_url) {
        Ok(response) => match response.json::<serde_json::Value>() {
            Ok(json) => return json["books"][0]["url_zip_file"].to_string(),
            Err(_) => return String::from("")
        }
        Err(_) => return String::from("")
    }
}