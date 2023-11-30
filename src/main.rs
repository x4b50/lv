use std::process::ExitCode;
use lv::{Lada, Inst};

const STACK_CAP: usize = 10;

fn main() -> ExitCode {
    let prog: Vec<Inst> = vec![
        Inst::push(69),
        Inst::push(420),
        Inst::plus(),
        Inst::push(440),
        Inst::minus(),
        Inst::push(2),
        Inst::mult(),
        Inst::push(14),
        Inst::div(),
        Inst::jmp(0),
        Inst::halt()
    ];

    // let mut vm = Lada::init(STACK_CAP);
    let mut vm = Lada::init::<STACK_CAP>(prog);

    while !vm.halted {
        match vm.exec_inst() {
            Ok(_) => {
                vm.print_stack();
            }
            Err(e) => {
                eprintln!("ERROR: {:?}, Instruciton: {}", e, vm.program[vm.ip]);
                eprintln!("{:?}", vm);
                return 1.into();
            }
        }
    }

    // println!("{}", std::mem::size_of::<InstType>());
    // println!("{}", std::mem::size_of::<Option<isize>>());
    // println!("{}", std::mem::size_of::<Inst>());
    0.into()
}
