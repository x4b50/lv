use std::process::ExitCode;

use lv::{Lada, Inst};

fn main() -> ExitCode {
    let mut vm = Lada::init(10);

    let prog: Vec<Inst> = vec![
        Inst::push(69),
        Inst::push(420),
        Inst::plus(),
        Inst::push(440),
        Inst::minus(),
        Inst::push(2),
        Inst::mult(),
        Inst::push(14),
        Inst::div()
    ];

    for i in 0..prog.len() {
        match vm.exec_inst(&prog[i]) {
            Ok(_) => {
                println!("{:?}", vm);
            }
            Err(e) => {
                eprintln!("ERROR: {:?}", e);
                eprintln!("{:?}", vm);
                return 1.into();
            }
        }
    }

    println!("{:?}", vm);
    0.into()
}
