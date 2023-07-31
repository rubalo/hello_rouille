use std::error::Error;
use std::fs;

pub struct Config {
    pub binary: String,
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String] ) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Invalid number of argument");
        }

        let binary = args[0].clone();
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {binary, query, file_path })
    }

}

pub fn run(config: Config) -> Result <(), Box<dyn Error>> {

    let content = fs::read_to_string(config.file_path)?;

    println!("{}", content);

    Ok(())
}