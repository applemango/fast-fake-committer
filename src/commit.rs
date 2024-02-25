use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use sha1::{Sha1, Digest};
use std::fs::{File, create_dir};
use std::path::Path;

pub fn write_bytes(path: String, bytes: Vec<u8>) {
    let mut file = File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
}

pub fn zlib_decode(bytes: &[u8]) -> String {
    let mut decoder = ZlibDecoder::new(bytes);
    let mut s = String::new();
    decoder.read_to_string(&mut s).unwrap();
    s
}
pub fn read_bytes(path: String) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    bytes
}

pub fn zlib_deflect(bytes: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&bytes).unwrap();
    let bytes = encoder.finish().unwrap();
    bytes
}

pub fn generate_hash(content: String, header: String) -> String {
    let store = content + &header;
    let mut hasher = Sha1::new();
    hasher.update(store.clone());
    let sha1 = hasher.finalize();
    let sha1_str = format!("{:x}", sha1);
    sha1_str
}


pub fn directory_is_exists(path: String) -> bool {
    Path::new(&path).exists()
}

pub fn create_path(base: String, hash: String) -> String {
    let directory = hash[0..2].to_string();
    let file = hash[2..].to_string();
    let directory_path = format!("{}/{}", base, directory);
    let p = format!("{}/{}", directory_path.clone(), file);
    if !directory_is_exists(directory_path.clone()) {
        let _ = create_dir(directory_path).unwrap();
    }
    p
}

pub fn create_base_header(tree: String, parent: String) -> String {
    let email = "applemango@example.com";
    let name = "applemango";
    let time = "1708735321 +0900";
    let author = format!("{} <{}> {}", name, email, time);
    let committer = author.clone();
    let f = format!("tree {}\nparent {}\nauthor {}\ncommitter {}\n\n", tree, parent, author, committer);
    f
}

pub fn create_header(tree: String, parent: String) -> String {
    let header = create_base_header(tree, parent);
    let bytes = header.as_bytes().len();
    let f = format!("commit {}\0{}", bytes, header);
    f
}

pub fn commit_empty(path: String, tree: String, parent: String) -> String {
    let header = create_header(tree, parent);
    let hash = generate_hash("".to_string(), header.clone());

    let bytes = zlib_deflect(header.as_bytes());

    write_bytes(create_path(path, hash.clone()), bytes);
    hash
}

pub fn read_commit(hash: String) -> String {
    let bytes = read_bytes(create_path("./assets".to_string(), hash));
    zlib_decode(&bytes)
}