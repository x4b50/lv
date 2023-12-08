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

    // for parsing args, include option tu use cpp, if so ofset the line on error
    // for i in 2..args.len() {
    // }

    let prog = match asm_parse(&source) {
        Ok(p) => p,
        Err((e,n)) => {
            eprintln!("Error on line {}: {:?}", n, e);
            return 1.into();
        }
    };

    match dump_prog_to_file(&prog, &args[2]) {
        Ok(_) => {}
        Err(e) => {
            eprint!("Error writing to file: {e}");
            return 1.into();
        }
    }

    0.into()
}
