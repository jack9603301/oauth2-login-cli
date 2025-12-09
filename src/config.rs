use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use serde::Serialize;
use std::clone::Clone;

#[derive(Serialize, Deserialize)]
pub struct Oauth2Config {
    pub provider: String,
    pub app_id: String,
    pub scopes: String,
    pub device_code_endpoint: String,
    pub token_endpoint: String,
    pub token: Option<Oauth2Token>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Oauth2Token {
    pub token_type: String,
    pub expires: u64,
    pub access_token: String,
    pub refresh_token: String,
}

pub fn openJson(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file");
    return file_contents;
}

pub fn saveJson(filename: &str, data: &Vec<Oauth2Config>) {
    let context = serde_json::to_string_pretty(&data).unwrap();
    std::fs::write(filename, context).unwrap();
}

pub fn parse(contents: &mut String) -> Vec<Oauth2Config> {
    let data: Vec<Oauth2Config> = serde_json::from_str(&contents).expect("Failure to parse configuration file");
    return data;
}
