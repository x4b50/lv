use std::{process::ExitCode, io::stdin};
use lv::{Lada, file::*, Inst, InstType, PrintType};

const HELP_PAGE: &str = "Lada Virtual machine

Usage: lv FILE [OPTIONS]
  -h, --help\tprint this page
  -d\t\trun in debug mode
  -D\t\trun in step debug mode
  -A\t\tdebug arena memory
  -s [size]\tset stack size
  -a [size]\tset arena size
  -f\t\tprint stack values as floating point
  -b\t\tprint values (stack & arena) as hexadecimal";

fn main() -> ExitCode {
    let prog;
    let mut stack_cap: usize = 32;
    let mut arena_size: usize = 32;
    let mut debug = false;
    let mut debug_step = false;
    let mut debug_arena = false;
    let mut print_type = PrintType::I64;

    {// arg parsing - no need to hold the copied string in mem
        let args: Vec<_> = std::env::args().collect();
        if args.len() < 2 {
            eprintln!("Not enough arguments:\nlv <source.lb> or lv --help for more information");
            return 1.into();
        }

        if args[1] == "--help" || args[1] == "-h" {
            println!("{HELP_PAGE}");
            return 0.into();
        }

        let source: String = args[1].clone();
        prog = match read_prog_from_file(&source) {
            Ok(p) => {p}
            Err(e) => {
                eprint!("Error while reading {source}: {e}");
                return 1.into();
            }
        };

        let mut i = 2;
        while i < args.len() {
            if args[i] == "-d" {debug=true}
            else if args[i] == "-D" {debug=true;debug_step=true}
            else if args[i] == "-A" {debug_arena=true}
            else if args[i] == "-f" {print_type = PrintType::F64}
            else if args[i] == "-b" {print_type = PrintType::HEX}
            else if args[i] == "-s" {
                i += 1;
                stack_cap = match args[i].parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Error while parsing stack size: {e}");
                        return 1.into();
                    }
                };
            }
            else if args[i] == "-a" {
                i += 1;
                arena_size = match args[i].parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Error while parsing arena size: {e}");
                        return 1.into();
                    }
                };
            }
            else {
                eprintln!("Error while parsing arguments\nhelp page:");
                println!("{HELP_PAGE}");
                return 1.into()
            }
            i += 1;
        }
    }

    let mut vm = Lada::init(prog, stack_cap, arena_size);
    let mut ip = 0;
    while !vm.halted() {
        match vm.exec_inst(&print_type) {
            Ok(_) => {
                if debug || debug_arena {print!("Inst: {}: {}    \t", ip, vm.inst(ip));}
                if debug {vm.print_stack(&print_type);}
                if debug_arena {print!("Arena memory: ");
                    match print_type {
                        PrintType::I64 => {println!("{:?}",  vm.get_arena());}
                        _ => {println!("{:x?}", vm.get_arena());}
                    }
                }
                if debug_step {
                    let mut s= String::new();
                    stdin().read_line(&mut s).unwrap();
                }ip = vm.ip()
            }
            Err(e) => {
                if debug {eprintln!("{:#?}", vm)}
                eprintln!("\nERROR: {:?}, Instruciton: {}", e,
                          if vm.prog_len() > vm.ip() {
                              format!("{}", vm.inst(vm.ip()).clone())
                          } else {
                              format!("Expected: {}", lv::inst!(HALT))
                          });
                return 1.into();
            }
        }
    }

    0.into()
}
