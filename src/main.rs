mod token;

use clap::Parser;
use core::panic;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value = "")]
    text: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("{}", args.text);

    dotenv::dotenv().ok();

    let token = match token::read_token() {
        Ok(t) => {
            serde_json::from_str::<token::TokenResponse>(t.as_str()).expect("failed to parse token")
        }
        Err(_) => match token::update_token().await {
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        },
    };

    match token::store_token(token).await {
        Ok(v) => v,
        Err(e) => panic!("ee{}", e),
    }
}
