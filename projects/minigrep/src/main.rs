use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let binary: String = args[0];
    let search_string: String = args[1];
    let search_path: String = args[3];

    println!("Searching {search_string} in {search_path}");

}
