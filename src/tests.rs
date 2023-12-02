#[cfg(test)]

#[macro_export]
macro_rules! prog {
    () => { vec![
            Inst::push(0),
            Inst::push(69),
            Inst::dup(),
            Inst::pick(2),
            Inst::add(),
            Inst::print(),
            Inst::jmp(2),
            Inst::halt(),
        ]
    };
}

pub mod tests{
    use crate::*;

    #[test]
    fn check_file_operations() {
        let dest: &str = "prog_inst.dat";
        let mut prog = prog!();
        let prog_cp = prog.clone();

        file::dump_prog_to_file(&mut prog, dest).unwrap();
        let prog = file::read_prog_from_file(dest).unwrap();
        for i in 0..prog_cp.len() {
            assert!(prog[i] == prog_cp[i]);
        }
    }

    #[test]
    fn check_asm_translate() {
        let source: &str = "push 0\npush 69\ndup\npick 2\nadd\n.\njmp 2\nhalt";
        let asm_prog = file::asm_parse(source).unwrap();
        let prog = prog!();
        for i in 0..prog.len() {
            assert!(prog[i] == asm_prog[i]);
        }
    }

    #[test]
    fn check_asm_translate_comment() {
        let source: &str = "push 0\npush 69  ;comment\ndup;___\n    ;   \npick 2\nadd\n.\njmp 2\nhalt";
        let asm_prog = file::asm_parse(source).unwrap();
        let prog = prog!();
        for i in 0..prog.len() {
            assert!(prog[i] == asm_prog[i]);
        }
    }
}
