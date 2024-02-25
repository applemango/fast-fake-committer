use std::path::Path;

use crate::commit::*;
use crate::head::*;
use crate::args::*;

mod commit;
mod head;
mod args;


fn create_commit_chain(path: String, tree: String, last_commit_id: String, n: i32) -> Vec<String> {
    let mut commit_ids = Vec::new();
    let mut last = last_commit_id;
    for _ in 0..n {
        let commit_id = commit_empty(
            path.clone(),
            tree.clone(),
            last,
        );
        commit_ids.push(commit_id.clone());
        last = commit_id;
    };
    return commit_ids;
}

fn committer(option: CommitterArgs) {
    let base = Path::new(&option.base_path);
    let head_path = base.join("logs/HEAD").to_str().unwrap().to_string();
    let refs_head_path = base.join("logs/refs/heads/master").to_str().unwrap().to_string();
    let object_path = base.join("objects").to_str().unwrap().to_string();
    let refs_heads_path = base.join("refs/heads/master").to_str().unwrap().to_string();

    let tree = option.tree;

    add_head_backslash(head_path.clone());
    copy_head(head_path.clone(), refs_head_path.clone());
    let mut last_commit_id = get_last_commit_id(head_path.clone());
    let mut i = 0;
    let n = 10000;
    loop {
        let commit_ids = create_commit_chain(object_path.clone(), tree.clone(), last_commit_id.clone(), n);
        append_head_core(head_path.clone(), last_commit_id.clone(), commit_ids.clone());
        append_head_core(refs_head_path.clone(), last_commit_id.clone(), commit_ids.clone());
        update_heads_ref(refs_heads_path.clone(), last_commit_id);
        last_commit_id = commit_ids.last().unwrap().to_string();
        i += n;
        println!("{}", i);
    }
}

fn main() {
    let option = get_option();
    committer(option);
}
