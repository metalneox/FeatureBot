use reqwest::{header, Client, Error};
use serde_json::{json, Value};
use std::env;
//use select::document::Document;
//use select::predicate::{Attr, Name, Predicate};

async fn get_img(url: String) -> Result<String, String> {
    let client = Client::new();
    //let resp = client.get(url).send().await?.text().await?;

    let url_img = url.clone();

    let resp = client.get(url).send().await.unwrap().status();

    if resp.is_success() {
        return Ok(url_img);
    } else {
        let custom_url = "https://demofree.sirv.com/nope-not-here.jpg".to_string();
        return Err(custom_url);
    }
}

pub async fn chatgpt(stringa: String) -> Result<String, Error> {
    let client2 = Client::new();

    //let url_img = url.clone();

    let json_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
          {
            "role": "system",
            "content": "You are a poetic assistant, skilled in explaining complex programming concepts with creative flair."
          },
          {
            "role": "user",
            "content": "Compose a poem that explains the concept of recursion in programming."
          }
        ]
    });

    //let api_key = "";
    let api_key = env::var("OPENIA").unwrap_or(String::from(""));

    let resp = client2
        .post("https://api.openai.com/v1/chat/completions")
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&json_body)
        .send()
        .await?
        .text()
        .await;
    resp
}

pub async fn ollama(stringa: String) -> Result<String, Error> {
    let client2 = Client::new();

    //let url_img = url.clone();

    let json_body = json!({
        "model": "phi3",
        "prompt": stringa,
        "format": "json",
        "stream": false
    });

    //let api_key = env::var("OLLAMA").unwrap_or(String::from(""));

    let resp = client2
        .post("http://localhost:11434/api/generate")
        //.header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&json_body)
        .send()
        .await?
        .text()
        .await;
    resp
}

pub async fn screenshot(streamer: String) -> String {
    let base = "https://static-cdn.jtvnw.net/previews-ttv/".to_string();
    //lowercase?
    let url = format!("{}live_user_{}-440x248.jpg", base, streamer.to_lowercase());

    println!("{}", url);
    let result = get_img(url).await;

    if result.is_ok() {
        let temp = result.unwrap();

        if temp != "".to_string() {
            return temp;
        }

        return "Streamer non trovato".to_string();
    }

    "".to_string()
}

