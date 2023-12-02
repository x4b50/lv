use std::{process::ExitCode, io::{self, Write}};
use lv::file::*;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments:\n./ldis <input.lb>");
        return 1.into();
    }

    let source: String = args[1].clone();
    let prog = match read_prog_from_file(&source) {
        Ok(p) => p,
        Err(e) => {
            eprint!("Error parsing file {source}: {e}");
            return 1.into();
        }
    };

    let mut prog_str: Vec<u8> = vec![];
    for isnt in prog.into_iter() {
        for char in isnt.to_string().chars() {
            prog_str.push(char as u8);
        }
    }

    match io::stdout().write_all(&prog_str){
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error while writing to stdout: {e}");
            return 1.into();
        }
    }

    0.into()
}
