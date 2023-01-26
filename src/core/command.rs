//commands
//

//hashmap init con i vari commandi
//
//

use std::collections::HashMap;

use caduceo;


//hasmap struttura sara          cmd        +       value     +  info(opzionale)

/// Commands

#[derive(Debug,PartialEq)]
pub struct Cmds {
    pub cmd:String,
    pub value:Option<String>,
}


impl Cmds {      


    pub fn run(self)-> Result<String,String>{

        //sarà un modulo a parte dove ci sono le varie features
        //let mut features: HashMap<String, fn(&'a str)->String> = HashMap::new();

        let mut features: HashMap<String, fn(String)->String> = HashMap::new();

        features.insert("!rot13".to_string(),caduceo::crypto::ciphers::rot13);

    
        if features.get(&self.cmd).is_some(){
            
            //TODO: Il controllo va bene ma se c'è una funzione senza parametro non viene gestito.
            if self.value.is_some() {

                let result = features.get(&self.cmd).unwrap()(self.value.unwrap());
                return Ok(result);
            }
            
            Ok("Manca il valore da passare".to_string())

        }else{
            Err("Funzionalità non trovata".to_string())
        }

    }

}


