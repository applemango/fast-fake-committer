use std::{fs::File, io::{Read, Write}, path::Path};

pub fn read_head(path: String) -> String {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

pub fn get_last_commit_id(path: String) -> String {
    let head = read_head(path);
    let heads = head.split("\n");
    let mut last = heads.clone().last().unwrap();
    if last == "" {
        last = heads.clone().nth(heads.count() - 2).unwrap();
    }
    let id = last.split(" ").nth(1).unwrap();
    id.to_string()
}

/*
 * # Example
 * last commit id, commit id, author, date, message
 * ```
 * ```
 */
pub fn create_head_message(last_commit_id: String, commit_id: String) -> String {
}

pub fn write_head(path: String, ids: Vec<String>) {
    let mut head = read_head(path.clone());
    let mut last = get_last_commit_id(path.clone());
    write_head_core(path, head, last, ids);
}

pub fn write_head_core(path: String, _head: String, last_commit_id: String, ids: Vec<String>) {
    let mut head = _head.clone();
    let mut last = last_commit_id;
    head.push_str("\n");
    for id in ids {
        head.push_str(&create_head_message(last.clone(), id.clone()));
        last = id;
    }
    std::fs::write(Path::new(&path), head).unwrap();
}

pub fn add_head_backslash(path: String) {
    let mut head = read_head(path.clone());
    if head.split("\n").last().unwrap() == "" {
        return;
    }
    head.push_str("\n");
    std::fs::write(Path::new(&path), head).unwrap();
}

pub fn append_head_core(path: String, last_commit_id: String, ids: Vec<String>) {
    let mut f = File::options().append(true).open(path).unwrap();
    for id in ids {
        write!(&mut f, "{}", create_head_message(last_commit_id.clone(), id.clone()));
    }
}

pub fn delete_file(path: String) {
    std::fs::remove_file(path);
}

pub fn copy_head(source_path: String, target_path: String) {
    let head = read_head(source_path);
    delete_file(target_path.clone());
    std::fs::write(Path::new(&target_path), head).unwrap();
}