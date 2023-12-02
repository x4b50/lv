use std::{process::ExitCode, fs};
use lv::{Lada, Inst, file::*};

const STACK_CAP: usize = 25;
const DEST: &str = "prog_inst.dat";
const SOURCE: &str = "code.lv";

fn main() -> ExitCode {
    /* prog initialisation
    let mut prog = vec![
        Inst::push(0),
        Inst::push(1),
        Inst::dup(),
        Inst::pick(2),
        Inst::add(),
        Inst::halt(),
        // Inst::jmp(2),
    ];
    // */

    /* debug for file writing
    let prog_cp = prog.clone();
    dump_prog_to_file(&mut prog, DEST).unwrap();
    for i in 0..prog.len() {
        assert!(prog[i] == prog_cp[i]);
    }
    let prog = read_prog_from_file(DEST).unwrap();
    for i in 0..prog_cp.len() {
        assert!(prog[i] == prog_cp[i]);
    }
    // */

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

    // /* run
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
    // */

    0.into()
}
