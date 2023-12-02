use std::{process::ExitCode, fs};
use lv::file::*;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments:\n./lc <input.lv> <output.lb>");
        return 1.into();
    }

    let mut source: String = args[1].clone();
    source = match fs::read_to_string(source) {
        Ok(f) => {f}
        Err(e) => {
            eprintln!("Error openig file: {e}");
            return 1.into();
        }
    };

    let mut prog = match asm_parse(&source) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing file: {:?}", e);
            return 1.into();
        }
    };

    match dump_prog_to_file(&mut prog, &args[2]) {
        Ok(_) => {}
        Err(e) => {
            eprint!("Error writing to file: {e}");
            return 1.into();
        }
    }

    0.into()
}
