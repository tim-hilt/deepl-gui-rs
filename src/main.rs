use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct Req {
    target_lang: &'static str,
    source_lang: &'static str,
    text: [&'static str; 1],
}

#[derive(Deserialize)]
struct T {
    text: String,
}

#[derive(Deserialize)]
struct Res {
    translations: [T; 1],
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = env::var("DEEPL_API_KEY").expect("DEEPL_API_KEY not set");

    let body = Req {
        source_lang: "DE",
        target_lang: "EN",
        text: ["Hallo, Welt!"],
    };

    let res: Res = reqwest::Client::new()
        .post("https://api-free.deepl.com/v2/translate")
        .json(&body)
        .header(AUTHORIZATION, format!("DeepL-Auth-Key {}", api_key))
        .send()
        .await?
        .json()
        .await?;

    println!("{}", res.translations[0].text);

    Ok(())
}
