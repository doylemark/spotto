use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

pub async fn update_token() -> Result<TokenResponse, reqwest::Error> {
    let auth_key = std::env::var("SPOTTO_KEY").expect("You must set the SPOTTO_KEY env variable");

    let auth_header = format!("Basic {}", auth_key);

    println!("Updating token!");
    let client = reqwest::Client::new();
    let res = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", auth_header)
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;

    Ok(res)
}

pub async fn store_token(token: TokenResponse) -> Result<(), std::io::Error> {
    let path = Path::new("./token.json");

    let mut token_file = File::create(path).expect("failed to create/write token.json");

    let ser = serde_json::to_string(&token).expect("Failed to serialize token");

    token_file.write_all(ser.as_bytes())
}

pub fn read_token() -> Result<String, std::io::Error> {
    std::fs::read_to_string("./token.json")
}
