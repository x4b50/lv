use std::thread::sleep;

use super::*;

pub const NATIVES: [Native;3] = [
    Lada::sys_print,
    Lada::str_print,
    Lada::sleep,
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

    fn sleep(&mut self) -> Result<(), ExecErr> {
        self.stack_size -= 1;
        sleep(std::time::Duration::from_millis(self.stack[self.stack_size]as u64));
        Ok(())
    }
}
