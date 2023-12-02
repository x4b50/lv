use std::process::ExitCode;
use lv::{Lada, Inst, file::*};

const STACK_CAP: usize = 25;
const DEST: &str = "prog_inst.dat";

fn main() -> ExitCode {
    let mut prog = vec![
        Inst::push(0),
        Inst::push(1),
        Inst::dup(),
        Inst::pick(2),
        Inst::plus(),
        Inst::halt(),
        Inst::jmp(2),
    ];

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

