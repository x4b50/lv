// #[allow(dead_code)]
#[cfg(test)]
mod tests;

use core::fmt;

#[derive(Debug)]
pub struct Lada {
    pub halted: bool,
    pub ip: usize,
    stack: Vec<isize>,
    stack_size: usize,
    pub program: Vec<Inst>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inst {
    kind: (InstType, bool),
    operand: isize
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
    IllegalInstAddr,
    IllegalOperand,
}

impl Lada {
    pub fn init(program: Vec<Inst>, stack_cap: usize) -> Lada {
        Lada {
            ip: 0,
            halted: false,
            stack: vec![0;stack_cap],
            stack_size: 0,
            program
        }
    }

    pub fn stack_print(&self) {
        print!("[");
        for i in 0..self.stack_size-1 {
            print!("{}, ", self.stack[i]);
        }
        println!("{}]", self.stack[self.stack_size-1]);
    }

    pub fn exec_inst(&mut self) -> Result<(), ExecErr> {
        if self.ip >= self.program.len() {
            return Err(ExecErr::IllegalInstAddr)
        }

        let inst = &self.program[self.ip];
        match inst.kind.0 {
            InstType::NOP => {}
            InstType::PUSH => {
                if self.stack_size >= self.stack.len() {
                    return Err(ExecErr::StackOverflow)
                }
                self.stack[self.stack_size] = inst.operand;
                self.stack_size += 1;
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
                if inst.operand < 0 || inst.operand >= self.stack_size as isize {
                    return Err(ExecErr::IllegalAddr);
                }
                self.stack[self.stack_size] = self.stack[self.stack_size -1 -inst.operand as usize];
                self.stack_size += 1;
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
                if inst.operand < 0 || inst.operand as usize >= self.program.len() {
                    return Err(ExecErr::IllegalInstAddr);
                }
                self.ip = inst.operand as usize;
                return Ok(())
            }

            InstType::JIF => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                if inst.operand < 0 || inst.operand as usize >= self.program.len() {
                    return Err(ExecErr::IllegalInstAddr);
                }
                if self.stack[self.stack_size-1] != 0 {
                    self.stack_size -= 1;
                    self.ip = inst.operand as usize;
                    return Ok(())
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
}

impl Inst {
    pub fn push(operand: isize) -> Inst {
        Inst { kind: (InstType::PUSH, true), operand}
    }
    pub fn pick(operand: isize) -> Inst {
        Inst { kind: (InstType::PICK, true), operand}
    }
    pub fn jmp(operand: isize) -> Inst {
        Inst { kind: (InstType::JMP, true), operand}
    }
    pub fn jmpif(operand: isize) -> Inst {
        Inst { kind: (InstType::JIF, true), operand}
    }

    pub fn nop() -> Inst {
        Inst { kind: (InstType::NOP, false), operand: 0 }
    }
    pub fn pop() -> Inst {
        Inst { kind: (InstType::POP, false), operand: 0 }
    }
    pub fn dup() -> Inst {
        Inst { kind: (InstType::DUP, false), operand: 0 }
    }
    pub fn add() -> Inst {
        Inst { kind: (InstType::ADD, false), operand: 0 }
    }
    pub fn sub() -> Inst {
        Inst { kind: (InstType::SUB, false), operand: 0 }
    }
    pub fn mult() -> Inst {
        Inst { kind: (InstType::MULT, false), operand: 0 }
    }
    pub fn div() -> Inst {
        Inst { kind: (InstType::DIV, false), operand: 0 }
    }
    pub fn eq() -> Inst {
        Inst { kind: (InstType::EQ, false), operand: 0 }
    }
    pub fn print() -> Inst {
        Inst { kind: (InstType::PRINT, false), operand: 0 }
    }
    pub fn dump() -> Inst {
        Inst { kind: (InstType::DUMP, false), operand: 0 }
    }
    pub fn halt() -> Inst {
        Inst { kind: (InstType::HALT, false), operand: 0 }
    }
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.kind.1 {
            write!(f, "{:?} {}", self.kind.0, self.operand)
        } else {
            write!(f, "{:?}", self.kind.0)
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
                    "nop" => {if operand != "" {return Err(ExecErr::IllegalOperand);} Inst::nop()}
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

                    "pop" => {if operand != "" {return Err(ExecErr::IllegalOperand);} Inst::pop()}
                    "dup" => {if operand != "" {return Err(ExecErr::IllegalOperand);} Inst::dup()}
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

                    "add" => {if operand != "" {return Err(ExecErr::IllegalOperand);} Inst::add()}
                    "sub" => {if operand != "" {return Err(ExecErr::IllegalOperand);} Inst::sub()}
                    "mult" => {if operand != "" {return Err(ExecErr::IllegalOperand);} Inst::mult()}
                    "div" => {if operand != "" {return Err(ExecErr::IllegalOperand);} Inst::div()}
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

                    "eq" => {if operand != "" {return Err(ExecErr::IllegalOperand);} Inst::eq()}
                    "print" | "." => {if operand != "" {return Err(ExecErr::IllegalOperand);} Inst::print()}
                    "dump" => {if operand != "" {return Err(ExecErr::IllegalOperand);} Inst::dump()}
                    "halt" => {if operand != "" {return Err(ExecErr::IllegalOperand);} Inst::halt()}

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
