use serde::Deserialize;

#[derive(Deserialize)]
pub struct deviceCodeEndpointResponse {
    pub user_code: String,
    pub device_code: String,
    pub verification_uri: String
}

#[derive(Deserialize)]
pub struct Error {
    pub error: String,
    pub error_description: String,
    pub error_codes: Vec<i32>
}

#[derive(Deserialize)]
pub struct tokenEndpointResponse {
    pub token_type: String,
    pub expires_in: i32,
    pub ext_expires_in: i32,
    pub access_token: String,
    pub refresh_token: String
}
