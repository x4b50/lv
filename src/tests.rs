#[cfg(test)]
pub mod tests{
    use crate::*;

    #[test]
    fn check_file_operations() {
        let dest: &str = "prog_inst.dat";
        let mut prog = vec![
            Inst::push(0),
            Inst::push(1),
            Inst::dup(),
            Inst::pick(2),
            Inst::add(),
            Inst::halt(),
            // Inst::jmp(2),
        ];
        let prog_cp = prog.clone();

        file::dump_prog_to_file(&mut prog, dest).unwrap();
        let prog = file::read_prog_from_file(dest).unwrap();
        for i in 0..prog_cp.len() {
            assert!(prog[i] == prog_cp[i]);
        }
    }

    #[test]
    fn check_asm_translate() {
        let source: &str = "push 0\npush 69\ndup\npick 2\nadd\nhalt\njmp 2";
        let asm_prog = file::asm_translate(source).unwrap();
        let prog = vec![
            Inst::push(0),
            Inst::push(69),
            Inst::dup(),
            Inst::pick(2),
            Inst::add(),
            Inst::halt(),
            Inst::jmp(2),
        ];
        for i in 0..prog.len() {
            assert!(prog[i] == asm_prog[i]);
        }
    }
}
