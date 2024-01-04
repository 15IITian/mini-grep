use std::fs;
use std::error::Error;
pub struct Config{
    pub query: String,
    pub file_path: String
}

impl Config{
pub fn  build(args: &[String]) -> Result<Config ,&str>{

    if args.len() < 3 {
       return Err("not enough arguments");
    }
    else if args.len() > 3{
        return Err("You have given unnecessay arguments");
    }
    let query= args[1].clone();
   let file_path= args[2].clone();

   Ok(Config{query,file_path})
}
}


pub fn run(config: Config) -> Result< (), Box<dyn Error>>{
   let content= fs::read_to_string(config.file_path)?;

    println!("the text is : \n{content}");   
    Ok(())        
}


