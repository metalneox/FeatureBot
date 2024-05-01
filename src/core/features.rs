use reqwest::{Client,header,Error};
use serde_json::{json, Value};
use std::env;
//use select::document::Document;
//use select::predicate::{Attr, Name, Predicate};

async fn get_img(url: String) -> Result<String,String> {
    let client = Client::new();
    //let resp = client.get(url).send().await?.text().await?;

    let url_img = url.clone();

    let resp = client.get(url).send().await.unwrap().status();

    if resp.is_success(){
        return Ok(url_img);
    }else{
        let custom_url = "https://demofree.sirv.com/nope-not-here.jpg".to_string();
        return Err(custom_url);
    }
     
    //let document = Document::from_read(resp.as_bytes()).unwrap();

    //for node in document.find(Attr("class", "preview").descendant(Name("img"))) {
    //    let src = node.attr("src").unwrap();
    //    return Ok(src.to_string());
    //}

    //Ok("".to_string())
}

pub async fn chatgpt(stringa: String)-> Result<String, Error>{
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
		//.status();
		//.await;
	//println!("->{:#?}",resp);
	//let v: Value = serde_json::from_str(resp);
	//resp.json().await.expect("REASON")
	//v
	resp

/*
curl https://api.openai.com/v1/chat/completions   -H "Content-Type: application/json"   -H "Authorization: Bearer $OPENAI_API_KEY"   -d '{
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
  }'
*/

    /*
    if resp.status().is_success() {
        // Leggi la risposta come JSON
        let response_json: Value = response.json().await?;
        println!("Risposta JSON:\n{}", response_json);
    } else {
        println!("La richiesta non è stata eseguita correttamente. Stato: {}", response.status());
    }*/


    /*
    if resp.unwrap().is_success(){
        //idea
        //return Ok(resp.body);
        let resp_json: Value = resp.json().await;
        println!("Risposta JSON:\n{}", resp_json);

        //return Ok("ciaone".to_string());
    }else{
        //return Err("Errore imprevisto".to_string());
        //
        println!("La richiesta non è stata eseguita correttamente. Stato: {}", resp.status());
    }
    */

}




pub async fn screenshot(streamer:String) -> String{
    let base = "https://static-cdn.jtvnw.net/previews-ttv/".to_string();
    //lowercase?
    let url = format!("{}live_user_{}-440x248.jpg",base,streamer.to_lowercase());

    println!("{}",url);
    let result = get_img(url).await;

    if result.is_ok(){ 

        let temp = result.unwrap();

        if temp != "".to_string(){
           return temp; 
        }

        return "Streamer non trovato".to_string();
    }

    "".to_string()

}


