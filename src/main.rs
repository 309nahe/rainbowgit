// Entry point for the CLI

mod blob;
use blob::{create_blob};

fn init_repo() {
    println!("Initializing repository...");
}

fn add_file(filename: &str) {
    match create_blob(filename) {
        Ok(blob) => println!("Blob created: {}", blob.hash),
        Err(e) => eprintln!("Error creatig blob: {}", e),
    }
}

fn commit(msg: &str) {
    println!("Committing with message: '{}'", msg);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "init" => init_repo(),
        "add" => add_file(&args[2]),
        "commit" => commit(&args[3]),
        _ => println!("Unknow command"),
    }
}
