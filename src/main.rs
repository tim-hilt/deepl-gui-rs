use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize)]
struct Res {
    text: String,
}

#[derive(Deserialize)]
struct Trans {
    translations: Vec<Res>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("DEEPL_API_KEY").expect("DEEPL_API_KEY not set");

    // TODO: Strongly type this!
    let body = json!({
        "text": ["Hallo, Welt!"],
        "target_lang": "EN",
        "source_lang": "DE"
    });

    let resp: Trans = reqwest::Client::new()
        .post("https://api-free.deepl.com/v2/translate")
        .json(&body)
        .header(AUTHORIZATION, format!("DeepL-Auth-Key {}", api_key))
        .send()
        .await?
        .json()
        .await?;

    if resp.translations.len() == 1 {
        println!("{}", resp.translations[0].text);
    } else {
        panic!("there should be a single translation");
    }

    Ok(())
}
