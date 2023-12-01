use std::process::ExitCode;
use lv::{Lada, Inst};

const STACK_CAP: usize = 25;

fn main() -> ExitCode {
    let prog: Vec<Inst> = vec![
        Inst::push(0),
        Inst::push(1),
        Inst::dup(),
        Inst::pick(2),
        Inst::plus(),
        Inst::jmp(2),
        // Inst::halt()
    ];

    // let mut vm = Lada::init(STACK_CAP);
    let mut vm = Lada::init::<STACK_CAP>(prog);

    while !vm.halted {
        // no ln causes glitch when error
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

    // println!("{}", std::mem::size_of::<InstType>());
    // println!("{}", std::mem::size_of::<Option<isize>>());
    // println!("{}", std::mem::size_of::<Inst>());
    0.into()
}
