use std::{collections::HashMap, future::Future};

use super::features::{screenshot, self};

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
        let mut features: HashMap<String,fn(String)-> String> = HashMap::new();
        
        features.insert("!rot13".to_string(), caduceo::crypto::ciphers::rot13);

        //

                                                //FIX: Questa funzione non verrà mai chiamat        
        let mut fet: HashMap<String,String> = HashMap::new();
        fet.insert("!info".to_string(),"Sono il mitico".to_string());

    
        if features.get(&self.cmd).is_some(){
            
            //TODO: Il controllo va bene ma se c'è una funzione senza parametro non viene gestito.
            if self.value.is_some() {
                //FIX: Controllare option
                
                if self.cmd == "!twitch" {
                    
                    let result = screenshot(self.value.unwrap()).await;

                    return Ok(result);

                }else{
                    let result = features.get(&self.cmd).unwrap()(self.value.unwrap());
                    return Ok(result);
 
                }


           }else{

                //let result = fet.get(&self.cmd).unwrap().to_string();

                let result = "to".to_string();
                return Ok(result);

            }
            
            //Ok("Manca il valore da passare".to_string())

        }else{
            Err("Funzionalità non trovata".to_string())
        }

    }

}


