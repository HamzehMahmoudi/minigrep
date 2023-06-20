use std::fs::read_to_string;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = read_to_string(config.file_path)?;
    let apperiance = search(config.query.as_str(), contents.as_str());
    println!("word {} appears in", config.query);
    for line in apperiance{
        println!("{}: {}", line.0, line.1)
    }
    Ok(())
}
pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String])-> Result<Config, &'static str>{
        if args.len() < 3 {
            return  Err("not enuogh arguments");
        }
        let query: String =  match args.get(1) {
            Some(f)=> f.clone(),
            None => String::from("")
        };
        let file_path: String= match args.get(2) {
            Some(f)=> f.clone(),
            None => String::from("")
        };
        if file_path == String::from("") || query ==String::from(""){
            return Err("invalid arguments")
        }else{
            Ok(Config {query, file_path})
        }
        
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)>{
    let mut apperiance: Vec<(usize ,&str)> = Vec::new();
    for line in contents.lines().enumerate(){
        if String::from(line.1).contains(query){
            apperiance.push((line.0+1 , line.1));
        }
    }
    apperiance
}