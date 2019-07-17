use std::env;
use std::process;
use minigrep;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }
    // println!("query: {}\nfilename: {} ", config.query, config.filename);

    // let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    // println!("\n\ncontent:{}\n", contents);
}
