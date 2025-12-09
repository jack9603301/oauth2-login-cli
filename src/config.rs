use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Oauth2Config {
    pub provider: String,
    pub app_id: String,
    pub scopes: String,
    pub device_code_endpoint: String,
    pub token_endpoint: String
}

pub fn openJson(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file");
    return file_contents;
}

pub fn parse(contents: &mut String) -> Vec<Oauth2Config> {
    let data: Vec<Oauth2Config> = serde_json::from_str(&contents).expect("Failure to parse configuration file");
    return data;
}
