mod config;
mod microsoft;
use std::env;
use reqwest::blocking::Client;
use std::collections::HashMap;
use serde_json;
use std::thread::sleep;
use std::time::Duration;

fn gen_token(config: &config::Oauth2Config) {
    println!("Use the following configuration:");
    println!("> app_id: {}", config.app_id);
    println!("> scopes: {}", config.scopes);
    println!("> device_code_endpoint: {}", config.device_code_endpoint);
    println!("> token_endpoint: {}", config.token_endpoint);

    let mut client = Client::new();
    let mut device_code_data = HashMap::new();
    device_code_data.insert("client_id", &config.app_id);
    device_code_data.insert("scope", &config.scopes);
            
    let device_code_response = client.post(&config.device_code_endpoint).form(&device_code_data).send().unwrap();

    if device_code_response.status().is_success() {
        let device_code_data: microsoft::deviceCodeEndpointResponse = serde_json::from_str(&device_code_response.text().unwrap()).expect("Failed to parse the data returned from the device code endpoint.");
        println!("Please check the interface data:");
        println!("> user code: {}", device_code_data.user_code);
        println!("> device_code: {}", device_code_data.device_code);
        println!("Please use your browser to visit {} and enter {} for verification.", device_code_data.verification_uri, device_code_data.user_code);

        loop {
            let mut token_data = HashMap::new();
            let grant_type = String::from("urn:ietf:params:oauth:grant-type:device_code");
            token_data.insert("grant_type", &grant_type);
            token_data.insert("client_id", &config.app_id);
            token_data.insert("device_code", &device_code_data.device_code);

            let token_response = client.post(&config.token_endpoint).form(&token_data).send().unwrap();

            if token_response.status().is_success() {
                let token_response_data: microsoft::tokenEndpointResponse = serde_json::from_str(&token_response.text().unwrap()).expect("Failed to parse the data returned from the token endpoint.");
                println!("Please check the interface data:");
                println!("> token type: {}", token_response_data.token_type);
                println!("> expires in: {}", token_response_data.expires_in);
                println!("> access token: {}", token_response_data.access_token);
                println!("> refresh token: {}", token_response_data.refresh_token);
                let token_config: config::Oauth2Token = config::Oauth2Token {
                    token_type: token_response_data.token_type,
                    access_token: token_response_data.access_token,
                    refresh_token: token_response_data.refresh_token
                };
                config.token = Some(token_config);
                break;
            } else {
                println!("Request error, please check your OAuth2 configuration.");
                let error: microsoft::Error = serde_json::from_str(&token_response.text().unwrap()).expect("Failed to parse the data returned from the token endpoint.");
                println!("Please check the interface data:");
                println!("> user code: {}", error.error);
                println!("> error description: {}", error.error_description);
                println!("> error codes: {}", error.error_codes.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "));
            }
            sleep(Duration::from_secs(1));
        }
    } else {
        println!("Request error, please check your OAuth2 configuration.");
        let error: microsoft::Error = serde_json::from_str(&device_code_response.text().unwrap()).expect("Failed to parse the data returned from the device code endpoint.");
        println!("Please check the interface data:");
        println!("> user code: {}", error.error);
        println!("> error description: {}", error.error_description);
        println!("> error codes: {}", error.error_codes.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "));
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = match args.get(2) {
        Some(arg) => arg,
        None => "gen-token"
    };
    let filename = match args.get(2) {
        Some(arg) => arg,
        None => {
            eprintln!("Provide configuration program file names");
            std::process::exit(1);
        }
    };
    let provider = match args.get(3) {
        Some(arg) => arg,
        None => {
            eprintln!("You must provide an OAuth2 provider!");
            std::process::exit(1);
        }
    };
    let mut contents = config::openJson(filename);
    let mut data = config::parse(&mut contents);

    for config in data.iter_mut() {
        if config.provider == *provider {
            if command == "gen-token" {
                gen_token(&config);
            }
        }
    }
    config::saveJson(filename, &data);

}
