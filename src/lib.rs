// #[allow(dead_code)]

use core::fmt;

#[derive(Debug)]
pub struct Lada {
    pub halted: bool,
    pub ip: usize,
    // stack: Vec<isize>,
    stack: Box<[isize]>,
    stack_size: usize,
    pub program: Vec<Inst>,
}

#[derive(Debug)]
pub struct Inst {
    kind: InstType,
    operand: Option<isize>,
}

#[derive(Debug)]
pub enum InstType {
    PUSH,
    PLUS,
    MINUS,
    MULT,
    DIV,
    JMP,
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
        let inst = &self.program[self.ip];
        match inst.kind {
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

            InstType::PLUS => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] += self.stack[self.stack_size-1];
                self.stack_size -= 1;
                self.stack[self.stack_size] = 0;
            }

            InstType::MINUS => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] -= self.stack[self.stack_size-1];
                self.stack_size -= 1;
                self.stack[self.stack_size] = 0;
            }

            InstType::MULT => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] *= self.stack[self.stack_size-1];
                self.stack_size -= 1;
                self.stack[self.stack_size] = 0;
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
                self.stack[self.stack_size] = 0;
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

            InstType::HALT => self.halted = true

            // _ => {
                // return Err(ExecErr::IllegalInst);
            // }
        }

        self.ip += 1;
        Ok(())
    }

    pub fn print_stack(&self) {
        println!("{:?}", self.stack);
    }

    pub fn print_stack_pretty(&self) {
        println!("{:#?}", self.stack);
    }
}

impl Inst {
    pub fn push(operand: isize) -> Inst {
        Inst { kind: InstType::PUSH, operand: Some(operand) }
    }
    pub fn jmp(operand: isize) -> Inst {
        Inst { kind: InstType::JMP, operand: Some(operand) }
    }

    pub fn plus() -> Inst {
        Inst { kind: InstType::PLUS, operand: None }
    }
    pub fn minus() -> Inst {
        Inst { kind: InstType::MINUS, operand: None }
    }
    pub fn mult() -> Inst {
        Inst { kind: InstType::MULT, operand: None }
    }
    pub fn div() -> Inst {
        Inst { kind: InstType::DIV, operand: None }
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
