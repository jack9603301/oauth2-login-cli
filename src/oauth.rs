use serde::Deserialize;

#[derive(Deserialize)]
pub struct deviceCodeEndpointResponse {
    pub user_code: String,
    pub device_code: String,
    #[serde(alias="verification_uri", alias="verification_url")]
    pub verification_uri: String
}

#[derive(Deserialize)]
pub struct Error {
    pub error: String,
    pub error_description: String
}

#[derive(Deserialize)]
pub struct tokenEndpointResponse {
    pub token_type: String,
    pub expires_in: i32,
    pub access_token: String,
    pub refresh_token: String
}
