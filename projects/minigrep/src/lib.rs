use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub binary: String,
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String] ) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Invalid number of argument");
        }

        let binary = args[0].clone();
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        Ok(Config {binary, query, file_path , ignore_case})
    }

}

pub fn run(config: Config) -> Result <(), Box<dyn Error>> {
    let content: String  = fs::read_to_string(config.file_path)?;

    let res: Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for found in res {
        println!("{}", found);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = vec![];
    for line in content.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.to_uppercase().contains(&query.to_uppercase()){
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_result() {
        let query = "Line three";
        let content = "\
Hello content
There is not that much here
Line three
Last line";

        assert_eq!(vec!["Line three"], search(query, content));
    }

    #[test]
    fn case_insensive_result() {
        let query = "Test";
        let content = "\
There are two lines
That contains test
But not with the same case
for TeStIng";

        assert_eq!(vec!["That contains test", "for TeStIng"], search_case_insensitive(query, content));
    }
}
