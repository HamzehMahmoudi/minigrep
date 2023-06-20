#[cfg(test)]
mod tests;

use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem during parsing arguments: {err}");
        process::exit(1)
    });
    // let query = conf.query;
    // let target_address = conf.file_path;
    // println!("searching for {} in {}", query, target_address);
    if let Err(e) = minigrep::run(conf){
        eprintln!("Application error {e}");
        process::exit(1)
    }

}
