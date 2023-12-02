use std::{process::ExitCode, fs};
use lv::{Lada, file::*};

const STACK_CAP: usize = 25;
const SOURCE: &str = "code.lv";

fn main() -> ExitCode {
    let source = match fs::read(SOURCE) {
        Ok(f) => match String::from_utf8(f) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Invalid utf-8: {e}");
                return 1.into();
            }
        },
        Err(e) => {
            eprintln!("Error openig file: {e}");
            return 1.into();
        }
    };

    let prog = match asm_translate(&source) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing file: {:?}", e);
            return 1.into();
        }
    };

    let mut vm = Lada::init::<STACK_CAP>(prog);

    while !vm.halted {
        print!("{}: {}    \t", vm.ip, vm.program[vm.ip]);
        match vm.exec_inst() {
            Ok(_) => {
                vm.stack_print();
            }
            Err(e) => {
                eprintln!("ERROR: {:?}, Instruciton: {}", e, vm.program[vm.ip]);
                eprintln!("{:?}", vm);
                return 1.into();
            }
        }
    }

    0.into()
}
