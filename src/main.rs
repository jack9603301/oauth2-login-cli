mod config;
mod microsoft;
use std::env;
use reqwest::blocking::Client;
use std::collections::HashMap;
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(arg) => arg,
        None => {
            eprintln!("Provide configuration program file names");
            std::process::exit(1);
        }
    };
    let provider = match args.get(2) {
        Some(arg) => arg,
        None => {
            eprintln!("You must provide an OAuth2 provider!");
            std::process::exit(1);
        }
    };
    let mut contents = config::openJson(filename);
    let data = config::parse(&mut contents);

    for config in data {
        if config.provider == *provider {
            println!("Use the following configuration:");
            println!("> app_id: {}", config.app_id);
            println!("> scopes: {}", config.scopes);
            println!("> device_code_endpoint: {}", config.device_code_endpoint);
            println!("> token_endpoint: {}", config.token_endpoint);

            let mut client = Client::new();
            let mut device_code_data = HashMap::new();
            device_code_data.insert("client_id", config.app_id);
            device_code_data.insert("scope", config.scopes);
            
            let device_code_response = client.post(config.device_code_endpoint).form(&device_code_data).send().unwrap();

            if device_code_response.status().is_success() {
                let device_code_data: microsoft::deviceCodeEndpointResponse = serde_json::from_str(&device_code_response.text().unwrap()).expect("Failed to parse the data returned from the device code endpoint.");
                println!("Please check the interface data:");
                println!("> user code: {}", device_code_data.user_code);
                println!("> device_code: {}", device_code_data.device_code);
                println!("Please use your browser to visit {} and enter {} for verification.", device_code_data.verification_uri, device_code_data.user_code)
            } else {
                println!("Request error, please check your OAuth2 configuration.");
                let error: microsoft::Error = serde_json::from_str(&device_code_response.text().unwrap()).expect("Failed to parse the data returned from the device code endpoint.");
                println!("Please check the interface data:");
                println!("> user code: {}", error.error);
                println!("> error description: {}", error.error_description);
                println!("> error codes: {}", error.error_codes.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "));
                return;
            }

        }
    }
}
