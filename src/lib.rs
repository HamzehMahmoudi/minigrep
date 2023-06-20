use std::fs::read_to_string;
use std::error::Error;
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let p = config.file_path.clone();
    let contents = read_to_string(config.file_path)?;
    let appearance: Vec<(usize, &str)>;
    if config.ignore_case{
        appearance = search_case_insensetive(config.query.as_str(), contents.as_str());
    }else{
        appearance = search(config.query.as_str(), contents.as_str());
    }
    if appearance.len() > 0 {
        println!("'{}' appears in:", config.query);
        for line in appearance{
            println!("{}: {}", line.0, line.1)
        }
    }else {
        println!("'{}' dosen't apeare in {}", config.query, p) ;  
    }
    Ok(())
}
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
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
            let var_env = env::var("IGNORE_CASE");
            let ignore_case:bool;
            match var_env {
                Ok(v)=> ignore_case = v.eq("1"),
                Err(_) => ignore_case = false                
            }
            Ok(Config {query, file_path, ignore_case})
        }
        
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)>{
    let mut appearance: Vec<(usize ,&str)> = Vec::new();
    for line in contents.lines().enumerate(){
        if String::from(line.1).contains(query){
            appearance.push((line.0+1 , line.1));
        }
    }
    appearance
}

pub fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)>{
    let mut appearance: Vec<(usize ,&str)> = Vec::new();
    for line in contents.lines().enumerate(){
        if line.1.to_lowercase().contains(query.to_lowercase().as_str()){
            appearance.push((line.0+1 , line.1));
        }
    }
    appearance
}