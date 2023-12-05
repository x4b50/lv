use std::process::ExitCode;
use lv::{Lada, file::*, Inst, PrintType};

fn main() -> ExitCode {
    let prog;
    let mut stack_cap: usize = 32;
    let mut debug = false;
    let mut print_type = PrintType::I64;

    {// arg parsing - no need to hold the copied string in mem
        let args: Vec<_> = std::env::args().collect();
        if args.len() < 2 {
            eprintln!("Not enough arguments:\n./lv <source.lb> optional: <stack capacity> -d (debug) -f (print stack as floats)");
            return 1.into();
        }

        let source: String = args[1].clone();
        prog = match read_prog_from_file(&source) {
            Ok(p) => {p}
            Err(e) => {
                eprint!("Error while reading {source}: {e}");
                return 1.into();
            }
        };

        for i in 2..args.len() {
            if args[i] == "-d" {debug=true}
            else if args[i] == "-f" {print_type = PrintType::F64}
            else if args[i] == "-p" {print_type = PrintType::PTR}
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
    }

    let mut vm = Lada::init(prog, stack_cap);
    let mut ip = 0;
    while !vm.halted {
        match vm.exec_inst(&print_type) {
            Ok(_) => {if debug {
                print!("Inst: {}: {}    \t", ip, vm.program[ip]);
                vm.stack_print(&print_type);
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
