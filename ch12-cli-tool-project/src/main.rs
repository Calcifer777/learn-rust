mod args;

use std::{env, process};
use ch12_cli_tool_project::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = SimpleGrepConfig::build(&args)
        .unwrap_or_else( |err| {
                println!("Problem parsing arguments: {err}");
                process::exit(1);
        });
    println!("File path: {:?}", cfg.filepath);
    println!("Query: {:?}", cfg.query);

    if let Err(e) = run(cfg) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

