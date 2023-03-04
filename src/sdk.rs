use std::env;

pub struct GPT {

}

impl GPT {
    pub fn setup(&self){
         match env::var("PATH") {
            Ok(val) => println!("PATH is {}", val),
            Err(e) => println!("couldn't read PATH: {}", e),
        }
    }
}