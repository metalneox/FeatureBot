use reqwest::{Client};
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


