use lv::{VM, Inst};

fn main() {
    let mut vm = VM::init(10);

    vm.exec_inst(Inst::push(69)).unwrap();
    println!("{:?}", vm);
    vm.exec_inst(Inst::push(420)).unwrap();
    println!("{:?}", vm);
    vm.exec_inst(Inst::plus()).unwrap();
    println!("{:?}", vm);
}
