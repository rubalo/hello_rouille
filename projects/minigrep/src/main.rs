use std::env;
use std::process;

use minigrep::Config;


fn main() {

    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(| err | {
        err_message(err);
        process::exit(1);
    });

    println!("*****************");
    println!("Running {}", config.binary);
    println!("Searching {} in {}", config.query, config.file_path);
    println!("*****************");

    if let Err(err) = minigrep::run(config) {
        err_message(&err.to_string());
        process::exit(1);
    };

}

fn err_message(err: &str) { 
    println!("{}", err);
    usage();
}

fn usage() {
    println!("Usage: \n\tminigrep <Search_String> <File_path>");
}