#[allow(dead_code)]

#[derive(Debug)]
pub struct VM {
    stack: Vec<usize>,
    stack_size: usize,
}

// pub probably for now
pub struct Inst {
    pub kind: InstType,
    pub operand: Option<usize>,
}

pub enum InstType {
    PUSH,
    PLUS,
    MINUS,
    MULT,
    DIV,
}

#[derive(Debug)]
pub enum ExecErr {
    StackOverflow,
    StackUnderflow,
    IllegalInst,
    DivByZero,
    NoOperand,
}

impl VM {
    pub fn init(cap: usize) -> VM {
        VM {
            stack: vec![0;cap],
            stack_size: 0,
        }
    }

    pub fn exec_inst(&mut self, inst: Inst) -> Result<(), ExecErr> {
        match inst.kind {
            InstType::PUSH => {
                if self.stack_size >= self.stack.capacity() {
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
                self.stack[self.stack_size-2] /= self.stack[self.stack_size-1];
                self.stack_size -= 1;
                self.stack[self.stack_size] = 0;
            }

            // _ => {
                // return Err(ExecErr::IllegalInst);
            // }
        }

        Ok(())
    }
}

impl Inst {
    pub fn push(operand: usize) -> Inst {
        Inst { kind: InstType::PUSH, operand: Some(operand) }
    }

    pub fn plus() -> Inst {
        Inst { kind: InstType::PLUS, operand: None }
    }
    pub fn minus() -> Inst {
        Inst { kind: InstType::MINUS, operand: None }
    }
    pub fn mult() -> Inst {
        Inst { kind: InstType::MINUS, operand: None }
    }
    pub fn div() -> Inst {
        Inst { kind: InstType::DIV, operand: None }
    }
}
