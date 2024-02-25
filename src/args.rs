use std::{env, process::exit};


pub struct CommitterArgs {
    pub base_path: String,
    pub tree: String,
}

pub fn get_option() -> CommitterArgs {
    let args: Vec<String> = env::args().collect();
    let mut option_name = "".to_string();
    let mut option = CommitterArgs {
        base_path: "".to_string(),
        tree: "".to_string(),
    };

    for arg in args {
        if arg.clone().starts_with("-") {
            option_name = arg;
            continue
        }
        if option_name == "-g" {
            option.base_path = arg;
            continue
        }
        if option_name == "-t" {
            option.tree = arg;
        }
        option_name = "".to_string();
    }

    if option.base_path == "" || option.tree == "" {
        println!("usage: fake-committer [-g git folder] [-t tree id]");
        exit(1);
    }

    option
}