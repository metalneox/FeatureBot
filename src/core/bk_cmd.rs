//pub fn add(left: usize, right: usize) -> usize {
//    left + right
//}


//use featurebot::core;
//pub mod core;

//use core;


pub mod core;
mod test;


use std::collections::HashMap;


//use featurebot::core;
//use crate::core::command::*;

//use featurebot::core::mind;
//
//
//

/// Commands

pub struct Cmds {
    cmd:&'static str,
    value:&'static str,
}


impl Cmds {      

    pub fn run(self){

        //sarà un modulo a parte
        let mut features = HashMap::new();

        features.insert("!rot13","valore");


        ////////////////////////////////////////////////////7

        if features.get(self.cmd).is_some(){
            //lancio funzione e ritorno valore
            // funzione(self.value)
        }else{
            //ritorno errore
        }


        //cerco il commando se lo trova il passo il secondo parametro e poi ritorno il valore
                    //is nel hashmap dei commandi allora gli passo il secondo parametro se no gli
        //do un errore che non ho trovato il commando
        //if self.cmd.
        //command::hello();


    }

}



#[cfg(test)]
mod tests {
    use super::*;

    use core as cuore;

    #[test]
    fn it_works() {
        let commandi = Cmds{cmd:"!rot13",
                            value:"prova"};

        //let four = commandi.echo();
        commandi.run();

        assert_eq!(4,4);
    }

    fn modules(){
        let result = cuore::prova();
        
        println!("{result}");


    }



}
