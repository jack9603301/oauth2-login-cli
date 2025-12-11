mod config;
mod microsoft;
use clap::{Command, arg};
use reqwest::blocking::Client;
use std::collections::HashMap;
use serde_json;
use std::thread::sleep;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::io;

fn get_unix_timestamp_plus_offset(offset_seconds: i64) -> u64 {
    let now = SystemTime::now();

    let current_duration = now.duration_since(UNIX_EPOCH)
        .expect("The system time is earlier than the UNIX epoch, so the calculation cannot be performed.");

    let final_system_time = if offset_seconds >= 0 {
        let offset = Duration::from_secs(offset_seconds as u64);
        now.checked_add(offset)
    } else {
        let offset = Duration::from_secs(offset_seconds.abs() as u64);
        now.checked_sub(offset)
    };

    let final_duration = final_system_time
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Time calculation overflow")).unwrap()
        .duration_since(UNIX_EPOCH)
        .expect("The calculated time is earlier than the UNIX epoch.");

    final_duration.as_secs()
}

fn gen_token(config: &mut config::Oauth2Config) {
    println!("Use the following configuration:");
    println!("> app_id: {}", config.app_id);
    println!("> scopes: {}", config.scopes);
    println!("> device_code_endpoint: {}", config.device_code_endpoint);
    println!("> token_endpoint: {}", config.token_endpoint);

    let  client = Client::new();
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
                    expires: get_unix_timestamp_plus_offset(token_response_data.expires_in.into()),
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

fn renew(config: &mut config::Oauth2Config) {
    println!("Use the following configuration:");
    println!("> app_id: {}", config.app_id);
    println!("> scopes: {}", config.scopes);
    println!("> device_code_endpoint: {}", config.device_code_endpoint);
    println!("> token_endpoint: {}", config.token_endpoint);
    let expires: u64 = get_unix_timestamp_plus_offset(0);
    let mut token: config::Oauth2Token = match &config.token {
        Some(token) => token.clone(),
        None => {
            println!("The token was not obtained. You need to run the `gen-token` command first to get the token!");
            std::process::exit(1);
        }
    };

    let  client = Client::new();
    if token.expires <= expires {
        println!("Provider {} is token has expired; automatically refreshing.", config.account_name);
        let mut form_data = HashMap::new();
        let grant_type: String = String::from("refresh_token");
        form_data.insert("grant_type", &grant_type);
        form_data.insert("client_id", &config.app_id);
        form_data.insert("refresh_token", &token.refresh_token);
        
        let response = client.post(&config.token_endpoint)
            .form(&form_data)
            .send()
            .unwrap();

        if response.status().is_success() {
            let token_response_data: microsoft::tokenEndpointResponse = serde_json::from_str(&response.text().unwrap()).expect("Failed to parse the data returned from the token endpoint.");
            println!("Please check the interface data:");
            println!("> token type: {}", token_response_data.token_type);
            println!("> expires in: {}", token_response_data.expires_in);
            println!("> access token: {}", token_response_data.access_token);
            println!("> refresh token: {}", token_response_data.refresh_token);
            let token_config: config::Oauth2Token = config::Oauth2Token {
                token_type: token_response_data.token_type,
                expires: get_unix_timestamp_plus_offset(token_response_data.expires_in.into()),
                access_token: token_response_data.access_token,
                refresh_token: token_response_data.refresh_token
            };
            config.token = Some(token_config);
            println!("Token refresh completed.")
        } else {
            println!("Token endpoint call failed!")
        }
    } else {
        println!("Provider {} is token has not expired and does not need to be refreshed.", config.account_name);
    }

}

fn get_access_token(config: &mut config::Oauth2Config) {
    let mut token: config::Oauth2Token = match &config.token {
        Some(token) => token.clone(),
        None => {
            println!("The token was not obtained. You need to run the `gen-token` command first to get the token!");
            std::process::exit(1);
        }
    };
    println!("{}", token.access_token);
}

fn main() {
    let args = Command::new("oauth2-login-cli").version("0.1.0").about("Automatically generate and poll for OAuth2 tokens for email clients.")
        .arg(arg!([config]).required(true).help("config filename"))
        .arg(arg!([account_name]).required(true).help("oauth2 account_name"))
        .arg(arg!(-c --command <command>).help("Operation commands"))
        .get_matches();
    let command = match args.get_one::<String>("command") {
        Some(arg) => arg,
        None => "gen-token"
    };
    let filename = match args.get_one::<String>("config") {
        Some(arg) => arg,
        None => {
            eprintln!("Provide configuration program file names");
            std::process::exit(1);
        }
    };
    let account_name = match args.get_one::<String>("account_name") {
        Some(arg) => arg,
        None => {
            eprintln!("You must provide an OAuth2 account_name!");
            std::process::exit(1);
        }
    };
    let mut contents = config::openJson(filename);
    let mut data = config::parse(&mut contents);

    let mut found = false;
    for config in data.iter_mut() {
        if config.account_name == *account_name {
            found = true;
            match command {
                "gen-token" => {
                    gen_token(config);
                },
                "renew" => {
                    renew(config);
                },
                "get_access_token" => {
                    get_access_token(config);
                },
                _ => {
                    println!("Error: The command does not exist. Please check the parameters!");
                    std::process::exit(1);
                }
            }
        }
    }

    if !found {
        println!("No OAuth2 account_name found. Please check the parameters!");
    } else {
        config::saveJson(filename, &data);
    }
}
