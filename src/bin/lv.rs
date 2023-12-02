use std::process::ExitCode;
use lv::{Lada, file::*};

const STACK_CAP: usize = 25;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments");
        return 1.into();
    }

    let source: String = args[1].clone();
    let prog = match read_prog_from_file(&source) {
        Ok(p) => {p}
        Err(e) => {
            eprint!("Error while reading {source}: {e}");
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
