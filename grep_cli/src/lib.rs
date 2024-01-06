use std::fs;
use std::error::Error;
use std::env;
pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case:bool
}






impl Config{
pub fn  build<T: Iterator<Item= String>>(mut args:T) -> Result<Config ,& 'static str>{

    args.next();

    let query= match args.next(){
        Some(arg) => arg,
        None => return Err("Didn't get a query string"),
    };

    
    let file_path= match args.next(){
        Some(args)=>args,
        None => return Err("didn't get the file path"),
    };

    let ignore_case= env::var("IGNORE_CASE").is_ok();
   Ok(Config{query,file_path, ignore_case})
}
}


pub fn run(config: Config) -> Result< (), Box<dyn Error>>{
   let content= fs::read_to_string(config.file_path)?;

   let results=if config.ignore_case {
    search_case_insensitive(&config.query, &content)
   }
   else{
    search(&config.query, &content)
   };

   for word in results{
         println!("{word}");
   }
    // println!("the text is : \n{content}");   
    Ok(())        
}

 pub fn search<'a>(query:& str, contents:& 'a str)-> Vec<& 'a str>{
 
    contents
            .split_whitespace().filter(|line| line.contains(query)).collect()
            
}

fn search_case_insensitive<'a>(query: &str, contents:& 'a str) -> Vec<& 'a str>{
    let query= query.to_lowercase();
    // let mut results= Vec::new();

    // for word in contents.split_whitespace(){
    //     if word.to_lowercase().contains(&query){
    //          results.push(word);
    //     }
    // }

    // results

    contents
           .split_whitespace().filter(|line| line.contains(&query)).collect()
}


//------------------------------------------------TEST---------------------------------------------------//

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query="\
        duct";
        let contents= 
        "Rust:
safe, fast, productive.
Pick three.
Duct tape.";

assert_eq!(vec!["productive."],search(query, contents));
    }



    #[test]
    fn case_insensitive(){
        let query = "rUSt";
        let contents= "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.";

     assert_eq!(vec!["Rust:", "Trust"], search_case_insensitive(query, contents));
    }
}



