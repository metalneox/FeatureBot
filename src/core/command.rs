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
    pub cmd:&'static str,
    pub value:&'static str,
}


impl Cmds {      


    pub fn run(self)-> Result<String,String>{

        //sarà un modulo a parte dove ci sono le varie features
        let mut features: HashMap<&str, fn(&'static str)->String> = HashMap::new();

        features.insert("!rot13",caduceo::crypto::ciphers::rot13);

    
        if features.get(self.cmd).is_some(){
           
            let result = features.get(self.cmd).unwrap()(self.value);

            Ok(result) 
            //Err("Non Funziona".to_string())
        }else{
            Err("Non Funziona".to_string())
        }

    }

}


