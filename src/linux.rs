use std::thread::sleep;

use super::*;

pub const NATIVES: [Native;6] = [
    Lada::sys_print,
    Lada::str_print,
    Lada::sleep,
    Lada::arena_malloc,
    Lada::native_malloc,
    Lada::native_free,
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

    fn arena_malloc(&mut self) -> Result<(), ExecErr> {
        let adr = self.arena.len();
        self.arena.extend_from_slice(&vec![0;self.stack[self.stack_size-1]as usize]);
        self.stack[self.stack_size-1] = adr as isize;
        Ok(())
    }

    fn native_malloc(&mut self) -> Result<(), ExecErr> {
        let mut found = false;
        let mut adr = 0;
        for i in 0..self.ext_mem.len() {
            if self.ext_mem[i] == None {
                println!("found");
                self.ext_mem[i] = Some(vec![0;self.stack[self.stack_size-1]as usize]);
                found = true;
                adr = i << 48;
                break
            }
        }
        if !found {
            adr = self.ext_mem.len() << 48;
            self.ext_mem.push(Some(vec![0;self.stack[self.stack_size-1]as usize]));
        }
        if adr == 0 {
            return Err(ExecErr::NativeError);
        }
        self.stack[self.stack_size-1] = adr as isize;
        Ok(())
    }

    fn native_free(&mut self) -> Result<(), ExecErr> {
        self.stack_size -= 1;
        let adr = (self.stack[self.stack_size] >> 48) as usize;
        self.ext_mem[adr] = None;
        Ok(())
    }
}
