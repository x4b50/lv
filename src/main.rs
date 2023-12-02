use std::{process::ExitCode, fs};
use lv::{Lada, file::*};

const STACK_CAP: usize = 25;
// const SOURCE: &str = "code.lv";

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments");
        return 1.into();
    }

    let mut source: String = args[1].clone();
    {
        source = match fs::read_to_string(source) {
            Ok(f) => {f}
            Err(e) => {
                eprintln!("Error openig file: {e}");
                return 1.into();
            }
        };
    }

    let prog = match asm_parse(&source) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing file: {:?}", e);
            return 1.into();
        }
    };

    let mut vm = Lada::init::<STACK_CAP>(prog);
    while !vm.halted {
        match vm.exec_inst() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("ERROR: {:?}, Instruciton: {}", e, vm.program[vm.ip]);
                eprintln!("{:?}", vm);
                return 1.into();
            }
        }
    }

    0.into()
}
