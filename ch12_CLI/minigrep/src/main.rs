use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // NOTE: we disregard args[0], which holds the executable's filepath
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for \'{}\'", query);
    println!("In file \'{}\'", filename);
}
