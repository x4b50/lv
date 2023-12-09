use super::*;

impl Lada {
    pub fn native(&mut self, n: usize, args: &[isize]) -> Result<(), ExecErr> {
        let nat: [Native; 1] = [Lada::sys_print];
        nat[n](self, args)
    }

    fn sys_print(&mut self, v: &[isize]) -> Result<(), ExecErr> {
        println!("{v:?}");
        Ok(())
    }
}
