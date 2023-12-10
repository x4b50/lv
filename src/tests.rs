#[macro_export]
macro_rules! prog {
    () => { vec![
            inst_op!(PUSH, 0),
            inst_op!(PUSH, 69),
            inst!(DUP),
            inst_op!(PUSH,2),
            inst!(PICK),
            inst!(ADD),
            inst!(PRINT),
            inst_op!(JMP, 2),
            inst!(HALT),
        ]
    };
}

#[cfg(test)]
pub mod tests{
    use crate::*;

    #[test]
    fn check_file_operations() {
        let dest: &str = "prog_inst.dat";
        let mut prog = Program {
            inst: prog!(),
            mem: vec![]
        };
        let prog_cp = prog.clone();

        file::dump_prog_to_file(&mut prog, dest).unwrap();
        let prog = file::read_prog_from_file(dest).unwrap();
        for i in 0..prog_cp.inst.len() {
            assert!(prog.inst[i] == prog_cp.inst[i]);
        }
    }

    #[test]
    fn check_asm_translate() {
        let source: &str = "push 0\npush 69\ndup\npush 2\npick\nadd\n.\njmp 2\nhalt";
        let asm_prog = file::asm_parse(source).unwrap();
        let prog = prog!();
        for i in 0..prog.len() {
            assert!(prog[i] == asm_prog.inst[i]);
        }
    }

    #[test]
    fn check_asm_translate_comment() {
        let source: &str = "push 0\npush 69  ;comment\ndup;___\n    ;   \npush 2\npick\nadd\n.\njmp 2\nhalt";
        let asm_prog = file::asm_parse(source).unwrap();
        let prog = prog!();
        for i in 0..prog.len() {
            assert!(prog[i] == asm_prog.inst[i]);
        }
    }
}
