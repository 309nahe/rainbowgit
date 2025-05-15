// Definition of blob struct

use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::{Write};
use std::io;
use std::path::Path;

pub struct Blob {
    pub content: Vec<u8>,
    pub hash: String,
}

pub fn create_blob(path: &str) -> std::io::Result<Blob> {
    let content = fs::read(path)?;
    let header = format!("blob {}\0", content.len());
    let mut full_data = Vec::new();

    full_data.extend(header.as_bytes());
    full_data.extend(&content);

    let mut hasher = Sha1::new();
    hasher.update(&full_data);
    let hash = hasher.finalize();
    let hex_hash = hex::encode(&hash);

    let dir = format!(".rainbowgit/objects/{}", &hex_hash[..2]);
    let file = format!("{}/{}", dir, &hex_hash[2..]);

    if !Path::new(".rainbowgit/objects").exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Repository not initialized. Run `rainbowgit init` first.",
        ));
    }
    
    fs::create_dir_all(&dir)?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&full_data)?;
    let compressed_data = encoder.finish()?;

    let mut out = File::create(&file)?;
    out.write_all(&compressed_data)?;

    Ok(Blob {
        hash: hex_hash,
        content,
    })
}
