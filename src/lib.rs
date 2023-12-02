// #[allow(dead_code)]
#[cfg(test)]
mod tests;

use core::fmt;

#[derive(Debug)]
pub struct Lada {
    pub halted: bool,
    pub ip: usize,
    stack: Box<[isize]>,
    stack_size: usize,
    pub program: Vec<Inst>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inst {
    kind: InstType,
    operand: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstType {
    NOP,
    PUSH,
    POP,
    DUP,
    PICK,
    ADD,
    SUB,
    MULT,
    DIV,
    JMP,
    JIF,
    EQ,
    PRINT,
    DUMP,
    HALT,
}

#[derive(Debug)]
pub enum ExecErr {
    StackOverflow,
    StackUnderflow,
    IllegalInst,
    DivByZero,
    NoOperand,
    IllegalAddr,
    IllegalOperand,
}

impl Lada {
    pub fn init<const STACK_SIZE: usize>(program: Vec<Inst>) -> Lada {
        Lada {
            ip: 0,
            halted: false,
            // stack: vec![0;stack_size],
            stack: Box::new([0;STACK_SIZE]),
            stack_size: 0,
            program
        }
    }

    pub fn exec_inst(&mut self) -> Result<(), ExecErr> {
        if self.ip >= self.program.len() {
            return Err(ExecErr::IllegalAddr)
        }

        let inst = &self.program[self.ip];
        match inst.kind {
            InstType::NOP => {}
            InstType::PUSH => {
                if self.stack_size >= self.stack.len() {
                    return Err(ExecErr::StackOverflow)
                }
                match inst.operand {
                    Some(v) => {
                        self.stack[self.stack_size] = v;
                        self.stack_size += 1;
                    }
                    None => return Err(ExecErr::NoOperand)
                }
            }

            InstType::POP => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack_size -= 1;
            }

            InstType::DUP => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                if self.stack_size >= self.stack.len() {
                    return Err(ExecErr::StackOverflow)
                }
                self.stack[self.stack_size] = self.stack[self.stack_size-1];
                self.stack_size += 1;
            }

            InstType::PICK => {
                if self.stack_size >= self.stack.len() {
                    return Err(ExecErr::StackUnderflow)
                }
                match inst.operand {
                    Some(v) => {
                        if v < 0 || v >= self.stack_size as isize {
                            return Err(ExecErr::IllegalAddr);
                        }
                        self.stack[self.stack_size] = self.stack[self.stack_size -1 -v as usize];
                        self.stack_size += 1;
                    }
                    None => return Err(ExecErr::NoOperand)
                }
            }

            InstType::ADD => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] += self.stack[self.stack_size-1];
                self.stack_size -= 1;
            }

            InstType::SUB => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] -= self.stack[self.stack_size-1];
                self.stack_size -= 1;
            }

            InstType::MULT => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] *= self.stack[self.stack_size-1];
                self.stack_size -= 1;
            }

            InstType::DIV => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                if self.stack[self.stack_size-1] == 0 {
                    return Err(ExecErr::DivByZero);
                }
                self.stack[self.stack_size-2] /= self.stack[self.stack_size-1];
                self.stack_size -= 1;
            }

            InstType::JMP => {
                match inst.operand {
                    Some(op) => {
                        if op < 0 {
                            return Err(ExecErr::IllegalAddr);
                        }
                        self.ip = op as usize;
                        return Ok(())
                    }
                    None => return Err(ExecErr::NoOperand)
                }
            }

            InstType::JIF => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                match inst.operand {
                    Some(op) => {
                        if op < 0 {
                            return Err(ExecErr::IllegalAddr);
                        }
                        if self.stack[self.stack_size-1] != 0 {
                            self.stack_size -= 1;
                            self.ip = op as usize;
                            return Ok(())
                        }
                    }
                    None => return Err(ExecErr::NoOperand)
                }
                self.stack_size -= 1;
            }

            InstType::EQ => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] = (self.stack[self.stack_size-1] == self.stack[self.stack_size-2]) as isize;
                self.stack_size -= 1;
            }

            InstType::PRINT => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                println!("{}", self.stack[self.stack_size-1]);
            }

            InstType::DUMP => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack_print();
            }

            InstType::HALT => self.halted = true
        }

        self.ip += 1;
        Ok(())
    }

    pub fn stack_print(&self) {
        print!("[");
        for i in 0..self.stack_size-1 {
            print!("{}, ", self.stack[i]);
        }
        println!("{}]", self.stack[self.stack_size-1]);
    }
}

impl Inst {
    pub fn push(operand: isize) -> Inst {
        Inst { kind: InstType::PUSH, operand: Some(operand) }
    }
    pub fn pick(operand: isize) -> Inst {
        Inst { kind: InstType::PICK, operand: Some(operand) }
    }
    pub fn jmp(operand: isize) -> Inst {
        Inst { kind: InstType::JMP, operand: Some(operand) }
    }
    pub fn jmpif(operand: isize) -> Inst {
        Inst { kind: InstType::JIF, operand: Some(operand) }
    }

    pub fn nop() -> Inst {
        Inst { kind: InstType::NOP, operand: None }
    }
    pub fn pop() -> Inst {
        Inst { kind: InstType::POP, operand: None }
    }
    pub fn dup() -> Inst {
        Inst { kind: InstType::DUP, operand: None }
    }
    pub fn add() -> Inst {
        Inst { kind: InstType::ADD, operand: None }
    }
    pub fn sub() -> Inst {
        Inst { kind: InstType::SUB, operand: None }
    }
    pub fn mult() -> Inst {
        Inst { kind: InstType::MULT, operand: None }
    }
    pub fn div() -> Inst {
        Inst { kind: InstType::DIV, operand: None }
    }
    pub fn eq() -> Inst {
        Inst { kind: InstType::EQ, operand: None }
    }
    pub fn print() -> Inst {
        Inst { kind: InstType::PRINT, operand: None }
    }
    pub fn dump() -> Inst {
        Inst { kind: InstType::DUMP, operand: None }
    }
    pub fn halt() -> Inst {
        Inst { kind: InstType::HALT, operand: None }
    }
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.operand {
            Some(o) => write!(f, "{:?} {}", self.kind, o),
            None => write!(f, "{:?}", self.kind)
        }
        
    }
}

pub mod file {
    use std::{fs, mem::size_of};
    use super::*;

    pub fn read_prog_from_file(source: &str) -> std::io::Result<Vec<Inst>> {
        let buff = fs::read(source)?;
        let len = size_of::<Inst>()/size_of::<u8>();
        let n = buff.len()/len;

        let prog_slice = &buff[0..n];
        let prog = unsafe {
            &*(prog_slice as *const [_] as *const [Inst])
        };
        Ok(prog.to_vec())
    }

    pub fn dump_prog_to_file(prog: &mut Vec<Inst>, dest: &str) -> std::io::Result<()> {
        // let _ = std::fs::remove_file(dest);
        std::fs::File::create(dest)?;

        match fs::OpenOptions::new().write(true).open(dest) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error opening file {e}");
                return Err(e);
            }
        }

        let mut f_buff: Vec<u8> = vec![];
        let len = size_of::<Inst>()/size_of::<u8>();
        let n = prog.len();

        for _ in 0..len {
            prog.push(Inst::halt());
        }
        for i in 0..n {
            let prog_slice = &prog[i..i+len];
            let buff = unsafe {
                &*(prog_slice as *const [_] as *const [u8])
            };
            for j in 0..len {
                f_buff.push(buff[j]);
            }
        }
        for _ in 0..len {
            prog.pop();
        }

        match fs::write(dest, &f_buff) {
            Ok(_) => {}
            Err(e) => {
                println!("Error writing to a file {dest}: {e}");
                return Err(e);
            }
        };
        Ok(())
    }

    // will have to change or it will become a piece of spaghetti
    pub fn asm_parse(source: &str) -> Result<Vec<Inst>, ExecErr> {
        let mut inst_vec: Vec<Inst> = vec![];
        for mut line in source.lines() {
            let mut inst = "";
            let mut operand = "";
            let mut char_count = 0;
            let mut comment_count = 0;

            for char in line.chars() {
                if char == ';' {
                    for char in line[0..char_count].chars().rev() {
                        if char != ' ' {
                            break;
                        }
                        comment_count += 1;
                    }
                    (line,_) = line.split_at(char_count-comment_count);
                }
                char_count += 1
            }

            if line.len() == 0 {continue}
            char_count = 0;

            for char in line.chars() {
                if char == ' ' {
                    (inst, operand) = line.split_at(char_count);
                    (_,operand) = operand.split_at(1);
                    break;
                }
                char_count += 1
            }

            if inst == "" {
                (inst,_) = line.split_at(line.len());
            }

            inst_vec.push(
                match inst {
                    "nop" => {Inst::nop()}
                    "push" => {
                        match operand.parse::<isize>() {
                            Ok(op) => {
                                Inst::push(op)
                            }
                            Err(e) => {
                                eprintln!("Error: {e}");
                                return Err(ExecErr::IllegalOperand);
                            }
                        }
                    }

                    "pop" => {Inst::pop()}
                    "dup" => {Inst::dup()}
                    "pick" => {
                        match operand.parse::<isize>() {
                            Ok(op) => {
                                Inst::pick(op)
                            }
                            Err(e) => {
                                eprintln!("Error: {e}");
                                return Err(ExecErr::IllegalOperand);
                            }
                        }
                    }

                    "add" => {Inst::add()}
                    "sub" => {Inst::sub()}
                    "mult" => {Inst::mult()}
                    "div" => {Inst::div()}
                    "jmp" => {
                        match operand.parse::<isize>() {
                            Ok(op) => {
                                Inst::jmp(op)
                            }
                            Err(e) => {
                                eprintln!("Error: {e}");
                                return Err(ExecErr::IllegalOperand);
                            }
                        }
                    }

                    "jmpif" | "jif" => {
                        match operand.parse::<isize>() {
                            Ok(op) => {
                                Inst::jmpif(op)
                            }
                            Err(e) => {
                                eprintln!("Error: {e}");
                                return Err(ExecErr::IllegalOperand);
                            }
                        }
                    }

                    "eq" => {Inst::eq()}
                    "print" | "." => {Inst::print()}
                    "dump" => {Inst::dump()}
                    "halt" => {Inst::halt()}

                    &_ => {
                        eprintln!("Error: Illegal instruction or I forgot to include some");
                        return Err(ExecErr::IllegalInst);
                    }
                }
            );

        }

        Ok(inst_vec)
    }
}
