use std::env;
use std::process;
use grep_cli::Config;
fn main(){


   
let config= Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{}", err);
        process::exit(1);
    });

 if let Err(e)=  grep_cli::run(config){
    println!("Application error : {e}");
    process::exit(1);
 };

}


