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
