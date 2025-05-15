// Entry point for the CLI

mod blob;
use blob::{create_blob};
use std::fs;
use std::path::Path;
use std::io;

fn init_repo() -> io::Result<()> {
    let base = ".rainbowgit";
    if Path::new(".rainbowgit").exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Repository already initialized",
        ));
    }
    fs::create_dir_all(&base)?;
    fs::create_dir_all(format!("{}/objects", base))?;
    fs::create_dir_all(format!("{}/refs/heads", base))?;

    fs::write(format!("{}/HEAD", base), "ref: refs/heads/main\n")?;

    println!("Initialized empty rainbowgit repository in {}", base);
    return Ok(())
}

fn add_file(filename: &str) -> io::Result<()> {
    match create_blob(filename) {
        Ok(blob) => println!("Blob created: {}", blob.hash),
        Err(e) => eprintln!("Error creatig blob: {}", e),
    }

    Ok(())
}

fn commit(msg: &str) -> io::Result<()> {
    println!("Committing with message: '{}'", msg);
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "init" => init_repo()?,
        "add" => add_file(&args[2])?,
        "commit" => commit(&args[3])?,
        _ => println!("Unknow command"),
    }

    Ok(())
}
