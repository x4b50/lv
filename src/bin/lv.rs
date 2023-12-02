use std::process::ExitCode;
use lv::{Lada, file::*, Inst};

fn main() -> ExitCode {
    let mut stack_cap: usize = 32;
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments:\n./lv <source.lb>");
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

    if args.len() > 2 {
        stack_cap = match args[2].parse::<usize>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error while parsing stack size: {e}");
                return 1.into();
            }
        };
    }

    let mut vm = Lada::init(prog, stack_cap);
    while !vm.halted {
        match vm.exec_inst() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("ERROR: {:?}, Instruciton: {}", e,
                          if vm.program.len() > vm.ip {
                              format!("{}", vm.program[vm.ip].clone())
                          } else {
                              format!("Expected: {}",Inst::halt())
                          });
                eprintln!("{:?}", vm);
                return 1.into();
            }
        }
    }

    0.into()
}
