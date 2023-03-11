use std::{collections::HashMap};

use super::features::{screenshot};

use std::future::Future;
use std::pin::Pin;

type AsyncFn = Pin<Box<dyn Future<Output = ()> + Send>>;

/// Commands

#[derive(Debug,PartialEq)]
pub struct Cmds {
    pub cmd:String,
    pub value:Option<String>,
}

impl Cmds {      


    pub async fn run(self)-> Result<String,String>{
  
        //INFO: Due hashmap dove una con parametri e una senza quella senza ritornano semplice
        //stringhe

        // per funzioni non async
        //let mut features_map: HashMap<String,fn(String)-> String> = HashMap::new();

        //per funzioni asyncrone
        let mut features_async: HashMap<String, AsyncFn> = HashMap::new();

        features_async.insert("!twitch".to_string(),Box::pin(async {screenshot("ciao".to_string());}));

        //per funzioni sync
        let mut features_sync: HashMap<String,fn(String)-> String> = HashMap::new();
        features_sync.insert("!rot13".to_string(), caduceo::crypto::ciphers::rot13 as fn(String) -> String);

        //println!("{:#?}",self);
    
        if features_async.get(&self.cmd).is_some(){
            
            //TODO: Il controllo va bene ma se c'è una funzione senza parametro non viene gestito.
            if self.value.is_some() {
                //FIX: Controllare option
                
                if self.cmd == "!twitch" {
                    
                    println!("{}",self.cmd);
                    let result = screenshot(self.value.unwrap()).await;

                    Ok(result)

                }else{
                    //let result = features_map.get(&self.cmd).unwrap()(self.value.unwrap());
                    //return Ok(result);
                    //
                    Ok("prova".to_string())
 
                }


           }else{

                //let result = fet.get(&self.cmd).unwrap().to_string();

                let result = "to".to_string();
                return Ok(result);

            }
            
        }else{

            if features_sync.get(&self.cmd).is_some(){
                let result = features_sync.get(&self.cmd).unwrap()(self.value.unwrap());
                Ok(result)
                
            }else {

                Err("Funzionalità non trovata".to_string())
            }

        }

    }

}


