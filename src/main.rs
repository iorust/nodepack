extern crate clap;

use std::process::Command;
use std::path::Path;
use clap::{Arg, App};

fn main() {
    let matches = App::new("nodepack")
        .version("0.0.1")
        .about("Compile Node.js application to a single executable.")
        .author("Qing Yan <admin@zensh.com>")
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .help("The Node.js application path to compile")
            .required(false))
        .get_matches();

    let mut path = "node_src";

    if let Some(_path) = matches.value_of("path") {
        path = _path;
    }

    let node_dir = Path::new(path);
    let child = Command::new("node")
        .current_dir(node_dir)
        .arg("nodepack")
        .spawn()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    child.wait_with_output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    println!("node process exit!");
}
