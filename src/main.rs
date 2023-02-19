use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct Req {
    target_lang: String,
    source_lang: String,
    text: [String; 1],
}

#[derive(Deserialize)]
struct T {
    text: String,
}

#[derive(Deserialize)]
struct Res {
    translations: [T; 1],
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = env::var("DEEPL_API_KEY").expect("DEEPL_API_KEY not set");

    let client = Client::new();

    let body = Req {
        source_lang: "DE".to_string(),
        target_lang: "EN".to_string(),
        text: ["Hallo, Welt!".to_string()],
    };

    let res: Res = client
        .post("https://api-free.deepl.com/v2/translate")
        .json(&body)
        .header(AUTHORIZATION, format!("DeepL-Auth-Key {}", api_key))
        .send()?
        .json()?;

    println!("{}", res.translations[0].text);

    Ok(())
}
