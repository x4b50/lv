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

    let mut i = 0;
    while i < prog.mem.len() {
        if i+8 <= prog.mem.len() {
            println!("@mem{} {}", i, isize::from_ne_bytes(match prog.mem[i..i+8].try_into() {
                Ok(v) => {v} Err(_) => {unreachable!()}
            }));
            i += 8;
        }
        else {
            let mut v: Vec<u8> = prog.mem[i..].to_vec();
            v.append(&mut vec![0u8;i+8-prog.mem.len()]);
            println!("@mem{} {}", i, isize::from_ne_bytes(match v[..].try_into() {
                Ok(v) => {v} Err(_) => {unreachable!()}
            }));
            break;
        };
    }

    let mut prog_str: Vec<u8> = vec![];
    for inst in prog.inst {
        for char in inst.to_string().chars() {
            prog_str.push(char as u8);
        }
        prog_str.push(b'\n');
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
