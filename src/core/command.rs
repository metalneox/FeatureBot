use std::collections::HashMap;

use super::features::{chatgpt, ollama, screenshot};

use std::future::Future;
use std::pin::Pin;

type AsyncFn = Pin<Box<dyn Future<Output = ()> + Send>>;

use serde_json::Value;
//use async_std::task;

/// Commands

#[derive(Debug, PartialEq)]
pub struct Cmds {
    pub cmd: String,
    pub value: Option<String>,
}

impl Cmds {
    pub async fn run(self) -> Result<String, String> {
        //per funzioni asyncrone
        let mut features_async: HashMap<String, AsyncFn> = HashMap::new();

        features_async.insert(
            "!twitch".to_string(),
            Box::pin(async {
                screenshot("ciao".to_string());
            }),
        );
        features_async.insert(
            "!gpt".to_string(),
            Box::pin(async {
                ollama("ciao".to_string());
            }),
        );

        //println!("{:#?}",prova);
        //per funzioni sync
        let mut features_sync: HashMap<String, fn(String) -> String> = HashMap::new();
        features_sync.insert(
            "!rot13".to_string(),
            caduceo::crypto::ciphers::rot13 as fn(String) -> String,
        );

        if features_async.get(&self.cmd).is_some() {
            //TODO: Il controllo va bene ma se c'è una funzione senza parametro non viene gestito.
            if self.value.is_some() {
                //FIX: Controllare option

                if self.cmd == "!twitch" {
                    let result = screenshot(self.value.unwrap()).await;

                    Ok(result)
                } else if self.cmd == "!gpt" {
                    let prova = ollama(self.value.unwrap()).await;

                    let value_ollama = match prova {
                        Ok(v) => v,
                        Err(v) => "Erroraccio".to_string(),
                    };
                    //FIX: Check in caso di Err di value_ollama
                    //FIX: Al momento ritornata una stringa non giusta
                    let val: Value = serde_json::from_str(&value_ollama).unwrap();
                    let result = val.get("response").and_then(|v| v.as_str()).unwrap_or("");
                    println!("->{}", result.to_string());

                    Ok(result.to_string())

                    //Ok("zio boia".to_string())
                } else {
                    Ok("prova".to_string())
                }
            } else {
                let result = "to".to_string();
                Ok(result)
            }
        } else {
            if features_sync.get(&self.cmd).is_some() {
                let result = features_sync.get(&self.cmd).unwrap()(self.value.unwrap());
                Ok(result)
            } else {
                Err("Funzionalità non trovata".to_string())
            }
        }
    }
}

