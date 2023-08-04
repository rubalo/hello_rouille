use std::env;
use std::process;

use minigrep::Config;


fn main() {

    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(| err | err_message(err));

    if let Err(err) = minigrep::run(config) {
        err_message(&err.to_string());
    };

}

fn err_message(err: &str) -> ! { 
    eprintln!("{}", err);
    usage();
    process::exit(1);
}

fn usage() {
    eprintln!("Usage: \n\tminigrep <Search_String> <File_path>");
}