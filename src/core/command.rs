use std::collections::HashMap;

use super::features::{chatgpt, ollama, screenshot};

use std::future::Future;
use std::pin::Pin;

type AsyncFn = Pin<Box<dyn Future<Output = ()> + Send>>;

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
        //FIX: Dovrebbe andare ma le api non andavano troppe richieste
        let prova = ollama("Cosa sei tu?".to_string()).await;
        println!("->{:#?}", prova);

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

