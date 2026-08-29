mod eval;
mod parser;
mod symtable;

use eval::eval;
use parser::parse;
use std::{env, fs};

fn parse_args() -> Result<Vec<String>, String> {
    let mut args = env::args();
    let prog = args.next().unwrap();
    let files: Vec<String> = args.collect();

    if files.len() == 0 {
        return Err(format!("usage: {prog} file1 [fileN]*"));
    }
    Ok(files)
}

fn eval_file(path: String) {
    eprint!("{path}: ");

    // read file
    let source = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return;
        }
    };

    // parse file
    eprintln!("\n");
    let pairs = parse(source.as_str());

    // evalulate and print result
    let res = eval(pairs);
    println!("result: {res}'");
}

fn main() {
    let args = parse_args();

    if let Err(error_msg) = args {
        eprintln!("{error_msg}");
        return;
    }

    for file in args.unwrap() {
        eval_file(file);
    }
}
