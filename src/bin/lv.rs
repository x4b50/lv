use std::process::ExitCode;
use lv::{Lada, file::*, Inst};

fn main() -> ExitCode {
    let mut stack_cap: usize = 32;
    let prog;
    let mut debug = false;

    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments:\n./lv <source.lb>");
        return 1.into();
    }

    {// no need to hold the copied string in mem
        let source: String = args[1].clone();
        prog = match read_prog_from_file(&source) {
            Ok(p) => {p}
            Err(e) => {
                eprint!("Error while reading {source}: {e}");
                return 1.into();
            }
        };
    }

    for i in 2..args.len() {
        if args[i] == "-d" {debug=true}
        else {
            stack_cap = match args[i].parse::<usize>() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error while parsing stack size: {e}");
                    return 1.into();
                }
            };
        }
    }

    let mut vm = Lada::init(prog, stack_cap);
    let mut ip = 0;
    while !vm.halted {
        match vm.exec_inst() {
            Ok(_) => {if debug {
                print!("{}: {}    \t", ip, vm.program[ip]);
                vm.stack_print();
            }ip = vm.ip}
            Err(e) => {
                if debug {eprintln!("{:?}", vm)}
                eprintln!("ERROR: {:?}, Instruciton: {}", e,
                          if vm.program.len() > vm.ip {
                              format!("{}", vm.program[vm.ip].clone())
                          } else {
                              format!("Expected: {}",Inst::halt())
                          });
                return 1.into();
            }
        }
    }

    0.into()
}
