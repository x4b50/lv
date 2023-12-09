use super::*;

pub const NATIVES: [Native;2] = [
    Lada::sys_print,
    Lada::str_print,
];

impl Lada {
    pub fn native(&mut self, n: usize) -> Result<(), ExecErr> {
        NATIVES[n](self)
    }

    // examlpe native function
    fn sys_print(&mut self) -> Result<(), ExecErr> {
        println!("{:?}", self.stack);
        Ok(())
    }

    fn str_print(&mut self) -> Result<(), ExecErr> {
        let len = self.stack[self.stack_size-1] as usize;
        let adr = self.stack[self.stack_size-2] as usize;
        self.stack_size -= 2;
        println!("{}", match std::str::from_utf8(&self.arena[adr..adr+len]) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error while parsing arena string: {e}");
                return Err(ExecErr::NativeError);
            }
        });
        Ok(())
    }
}
