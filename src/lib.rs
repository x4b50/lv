// #[allow(dead_code)]
#[cfg(test)]
mod tests;
use core::fmt;
use std::mem::transmute;

macro_rules! no_op_err {
    ($op:ident, $line:ident) => {
        if $op != "" {return Err((ExecErr::IllegalOperand, $line));}
    };
}

macro_rules! f64 {
    ($dest:expr, $op:tt, $source:expr) => {
        unsafe{ $dest = transmute::<f64, isize>(transmute::<isize, f64>($dest) $op transmute::<isize, f64>($source));}
        // $dest = (f64::from_bits($s1 as u64) $op f64::from_bits($s2 as u64)).to_bits() as isize;
    };
}

macro_rules! f64_bool {
    ($dest:expr, $op:tt, $source:expr) => {
        unsafe{ $dest = (transmute::<isize, f64>($dest) $op transmute::<isize, f64>($source)) as isize;}
    };
}

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
    SHOVE,
    ADD,
    SUB,
    MULT,
    DIV,
    ADDF,
    SUBF,
    MULTF,
    DIVF,
    SHL,
    SHR,
    AND,
    OR,
    XOR,
    NOT,
    /*
    ptr,
    ptrn
    // */
    JMP,
    JIF,
    EQ,
    NEQ,
    LT,
    GT,
    EQF,
    NEQF,
    LTF,
    GTF,
    PRINT,
    SHOUT,
    DUMP,
    EMPTY,
    IFEMPTY,
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

pub enum PrintType {
    I64,
    F64,
    PTR
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
    pub fn stack_print(&self, t: &PrintType) {
        print!("[");
        if self.stack_size == 0 {
            println!("]");
        } else {
            match t {
                PrintType::I64 => {
                    for i in 0..self.stack_size-1 {
                        print!("{}, ", self.stack[i]);
                    }   println!("{}]", self.stack[self.stack_size-1]);
                }
                PrintType::F64 => { unsafe {
                    for i in 0..self.stack_size-1 {
                        print!("{:.7e}, ",  transmute::<isize, f64>(self.stack[i]));
                    }   println!("{:.7e}]", transmute::<isize, f64>(self.stack[self.stack_size-1]));
                }}
                PrintType::PTR => {
                    for i in 0..self.stack_size-1 {
                        print!("{:x}, ", self.stack[i]);
                    }   println!("{:x}]", self.stack[self.stack_size-1]);
                }
            }
        }
    }

    pub fn exec_inst(&mut self, print_type: &PrintType) -> Result<(), ExecErr> {
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
                    return Err(ExecErr::StackOverflow)
                }
                if self.stack[self.stack_size-1] < 0 || self.stack[self.stack_size-1] >= self.stack_size as isize {
                    return Err(ExecErr::IllegalAddr);
                }
                self.stack[self.stack_size-1] = self.stack[self.stack_size -1 -self.stack[self.stack_size-1] as usize];
            }

            InstType::SHOVE => {
                if self.stack[self.stack_size-1] < 0 || self.stack[self.stack_size-1] >= self.stack_size as isize -1 {
                    return Err(ExecErr::IllegalAddr);
                }
                let adr = self.stack_size -2 -self.stack[self.stack_size-1]as usize;
                self.stack[adr] = self.stack[self.stack_size -2];
                self.stack_size -= 2;
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

            InstType::ADDF => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                f64!(self.stack[self.stack_size-2], +, self.stack[self.stack_size-1]);
                self.stack_size -= 1;
            }

            InstType::SUBF => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                f64!(self.stack[self.stack_size-2], -, self.stack[self.stack_size-1]);
                self.stack_size -= 1;
            }

            InstType::MULTF => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                f64!(self.stack[self.stack_size-2], *, self.stack[self.stack_size-1]);
                self.stack_size -= 1;
            }

            InstType::DIVF => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                f64!(self.stack[self.stack_size-2], /, self.stack[self.stack_size-1]);
                self.stack_size -= 1;
            }

            InstType::SHL => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] <<= self.stack[self.stack_size-1];
                self.stack_size -= 1;
            }
            InstType::SHR => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] >>= self.stack[self.stack_size-1];
                self.stack_size -= 1;
            }
            InstType::AND => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] &= self.stack[self.stack_size-1];
                self.stack_size -= 1;
            }
            InstType::OR => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] |= self.stack[self.stack_size-1];
                self.stack_size -= 1;
            }
            InstType::XOR => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] ^= self.stack[self.stack_size-1];
                self.stack_size -= 1;
            }
            InstType::NOT => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-1] = !self.stack[self.stack_size-1];
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
                self.stack[self.stack_size-2] = (self.stack[self.stack_size-2] == self.stack[self.stack_size-1]) as isize;
                self.stack_size -= 1;
            }

            InstType::NEQ => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] = (self.stack[self.stack_size-2] != self.stack[self.stack_size-1]) as isize;
                self.stack_size -= 1;
            }

            InstType::LT => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] = (self.stack[self.stack_size-2] < self.stack[self.stack_size-1]) as isize;
                self.stack_size -= 1;
            }

            InstType::GT => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-2] = (self.stack[self.stack_size-2] > self.stack[self.stack_size-1]) as isize;
                self.stack_size -= 1;
            }

            InstType::EQF => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                f64_bool!(self.stack[self.stack_size-2], ==, self.stack[self.stack_size-1]);
                self.stack_size -= 1;
            }

            InstType::NEQF => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                f64_bool!(self.stack[self.stack_size-2], !=, self.stack[self.stack_size-1]);
                self.stack_size -= 1;
            }

            InstType::LTF => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                f64_bool!(self.stack[self.stack_size-2], <, self.stack[self.stack_size-1]);
                self.stack_size -= 1;
            }

            InstType::GTF => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                }
                f64_bool!(self.stack[self.stack_size-2], >, self.stack[self.stack_size-1]);
                self.stack_size -= 1;
            }

            InstType::PRINT => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                unsafe { println!("{i} | {i:x} | {f:.7e}", i=self.stack[self.stack_size-1], f=transmute::<isize, f64>(self.stack[self.stack_size-1])); }
            }

            InstType::SHOUT => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                println!("{}", self.stack[self.stack_size-1]);
                self.stack_size -= 1;
            }

            InstType::DUMP => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                print!("Stack: ");
                self.stack_print(&print_type);
            }

            InstType::EMPTY => {
                self.stack_size = 0;
            }

            InstType::IFEMPTY => {
                if self.stack_size == 0 {
                    self.stack[0] = 1;
                } else {
                    self.stack[self.stack_size] = 0;
                }
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
    pub fn pick() -> Inst {
        Inst { kind: (InstType::PICK, false), operand: 0}
    }
    pub fn shove() -> Inst {
        Inst { kind: (InstType::SHOVE, false), operand: 0}
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
    pub fn addf() -> Inst {
        Inst { kind: (InstType::ADDF, false), operand: 0 }
    }
    pub fn subf() -> Inst {
        Inst { kind: (InstType::SUBF, false), operand: 0 }
    }
    pub fn multf() -> Inst {
        Inst { kind: (InstType::MULTF, false), operand: 0 }
    }
    pub fn divf() -> Inst {
        Inst { kind: (InstType::DIVF, false), operand: 0 }
    }
    pub fn shl() -> Inst {
		Inst { kind: (InstType::SHL, false), operand: 0 }
	}
    pub fn shr() -> Inst {
		Inst { kind: (InstType::SHR, false), operand: 0 }
	}
    pub fn and() -> Inst {
		Inst { kind: (InstType::AND, false), operand: 0 }
	}
    pub fn or() -> Inst {
		Inst { kind: (InstType::OR, false), operand: 0 }
	}
    pub fn xor() -> Inst {
		Inst { kind: (InstType::XOR, false), operand: 0 }
	}
    pub fn not() -> Inst {
		Inst { kind: (InstType::NOT, false), operand: 0 }
	}
    pub fn eq() -> Inst {
        Inst { kind: (InstType::EQ, false), operand: 0 }
    }
    pub fn neq() -> Inst {
        Inst { kind: (InstType::NEQ, false), operand: 0 }
    }
    pub fn lt() -> Inst {
        Inst { kind: (InstType::LT, false), operand: 0 }
    }
    pub fn gt() -> Inst {
        Inst { kind: (InstType::GT, false), operand: 0 }
    }
    pub fn eqf() -> Inst {
        Inst { kind: (InstType::EQF, false), operand: 0 }
    }
    pub fn neqf() -> Inst {
        Inst { kind: (InstType::NEQF, false), operand: 0 }
    }
    pub fn ltf() -> Inst {
        Inst { kind: (InstType::LTF, false), operand: 0 }
    }
    pub fn gtf() -> Inst {
        Inst { kind: (InstType::GTF, false), operand: 0 }
    }
    pub fn print() -> Inst {
        Inst { kind: (InstType::PRINT, false), operand: 0 }
    }
    pub fn shout() -> Inst {
        Inst { kind: (InstType::SHOUT, false), operand: 0 }
    }
    pub fn dump() -> Inst {
        Inst { kind: (InstType::DUMP, false), operand: 0 }
    }
    pub fn empty() -> Inst {
        Inst { kind: (InstType::EMPTY, false), operand: 0 }
    }
    pub fn ifempty() -> Inst {
        Inst { kind: (InstType::IFEMPTY, false), operand: 0 }
    }
    pub fn halt() -> Inst {
        Inst { kind: (InstType::HALT, false), operand: 0 }
    }


    pub fn to_string(&self) -> String {
        match self.kind.0 {
            InstType::NOP   => {format!("nop\n")}
            InstType::PUSH  => {format!("push {}\n", self.operand)}
            InstType::POP   => {format!("pop\n")}
            InstType::DUP   => {format!("dup\n")}
            InstType::PICK  => {format!("pick\n")}
            InstType::SHOVE => {format!("shove\n")}
            InstType::ADD   => {format!("add\n")}
            InstType::SUB   => {format!("sub\n")}
            InstType::MULT  => {format!("mult\n")}
            InstType::DIV   => {format!("div\n")}
            InstType::ADDF  => {format!("addf\n")}
            InstType::SUBF  => {format!("subf\n")}
            InstType::MULTF => {format!("multf\n")}
            InstType::DIVF  => {format!("divf\n")}
            InstType::SHL	=> {format!("shl\n")}
            InstType::SHR	=> {format!("shr\n")}
            InstType::AND	=> {format!("and\n")}
            InstType::OR	=> {format!("or\n")}
            InstType::XOR	=> {format!("xor\n")}
            InstType::NOT	=> {format!("not\n")}
            InstType::JMP   => {format!("jmp {}\n", self.operand)}
            InstType::JIF   => {format!("jmpif {}\n", self.operand)}
            InstType::EQ    => {format!("eq\n")}
            InstType::NEQ   => {format!("neq\n")}
            InstType::LT    => {format!("lt\n")}
            InstType::GT    => {format!("gt\n")}
            InstType::EQF   => {format!("eqf\n")}
            InstType::NEQF  => {format!("neqf\n")}
            InstType::LTF   => {format!("ltf\n")}
            InstType::GTF   => {format!("gtf\n")}
            InstType::PRINT => {format!("print\n")}
            InstType::SHOUT => {format!("shout\n")}
            InstType::DUMP  => {format!("dump\n")}
            InstType::EMPTY => {format!("empty\n")}
            InstType::IFEMPTY=>{format!("ifempty\n")}
            InstType::HALT  => {format!("halt\n")}
        }
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
        // assertion that the file has the right length
        assert!(buff.len()%size_of::<Inst>()==0);
        let len = size_of::<Inst>()/size_of::<u8>();
        let n = buff.len()/len;

        let prog_slice = &buff[0..n];
        let prog = unsafe {
            // TODO: use that
            // f64::from_ne_bytes()
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

    #[derive(Debug)]
    struct Label {
        name: String,
        addr: usize
    }

    #[derive(Debug)]
    struct Constant {
        name: String,
        value: isize
    }

    // will have to change or it will become a piece of spaghetti
    pub fn asm_parse(source: &str) -> Result<Vec<Inst>, (ExecErr, usize)> {
        let mut line_count = 0;
        let mut inst_vec: Vec<Inst> = vec![];
        let mut label_vec: Vec<Label> = vec![];
        let mut jmp_vec: Vec<&str> = vec![];
        let mut const_vec: Vec<Constant> = vec![];
        let mut sub_vec: Vec<&str> = vec![];

        for mut line in source.lines() {
            line_count += 1;
            let mut inst = "";
            let mut operand = "";
            let mut char_count = 0;
            let mut comment_count = 0;
            let mut label_count = 0;

            for char in line.chars() {
                if char == ';' {
                    for char in line[0..char_count].chars().rev() {
                        if char != ' ' {
                            break;
                        }
                        comment_count += 1;
                    }
                    (line,_) = line.split_at(char_count-comment_count);
                    break;
                }
                char_count += 1
            }

            char_count = 0;
            for char in line.chars() {
                if char == ':' {
                    for char in line[0..char_count].chars().rev() {
                        if !char.is_alphabetic() {
                            break;
                        }
                        label_count += 1;
                    }
                    label_vec.push(Label {
                        name: line[char_count-label_count..char_count].to_string(),
                        addr: inst_vec.len()
                    });
                    (_,line) = line.split_at(char_count+1);
                    break;
                }
                char_count += 1
            }
            
            char_count = 0;
            for char in line.chars() {
                if char != ' ' {
                    (_,line) = line.split_at(char_count);
                    break;
                }
                char_count += 1;
            }

            char_count = 0;
            if line.starts_with('$') {
                for char in line.chars() {
                    if char == ' ' {
                        let (const_name, mut value) = line.split_at(char_count);
                        (_,value) = value.split_at(1);
                        const_vec.push(Constant{
                            name: const_name.to_string(),
                            value: match value.parse::<isize>() {
                                Ok(v) => v,
                                Err(_) => {
                                    match value.parse::<f64>() {
                                        Ok(v) => unsafe {transmute::<f64, isize>(v)},
                                        Err(e) => {
                                            eprintln!("Invalid argument in macro definition: {e}");
                                            return Err((ExecErr::IllegalOperand, line_count));
                                        }
                                    }
                                }
                            }
                        });
                        line = "";
                        break;
                    }
                    char_count += 1
                }
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
                    "nop" => {no_op_err!(operand, line_count); Inst::nop()}
                    "push" => {
                        match operand.parse::<isize>() {
                            Ok(op) => {
                                Inst::push(op)
                            }
                            Err(_) => {
                                match operand.parse::<f64>() {
                                    Ok(op) => {
                                        Inst::push(unsafe { transmute::<f64, isize>(op)})
                                    }
                                    Err(_) => {
                                        sub_vec.push(&operand);
                                        Inst {
                                            kind: (InstType::PUSH, false),
                                            operand: 0
                                        }
                                    }
                                }
                            }
                        }
                    }

                    "pop" => {no_op_err!(operand, line_count); Inst::pop()}
                    "dup" => {no_op_err!(operand, line_count); Inst::dup()}
                    "pick"=> {no_op_err!(operand, line_count); Inst::pick()}
                    "shove"=>{no_op_err!(operand, line_count); Inst::shove()}
                    "add" | "+" => {no_op_err!(operand, line_count); Inst::add()}
                    "sub" | "-" => {no_op_err!(operand, line_count); Inst::sub()}
                    "mult"| "*" => {no_op_err!(operand, line_count); Inst::mult()}
                    "div" | "/" => {no_op_err!(operand, line_count); Inst::div()}
                    "addf" | "+f" => {no_op_err!(operand, line_count); Inst::addf()}
                    "subf" | "-f" => {no_op_err!(operand, line_count); Inst::subf()}
                    "multf"| "*f" => {no_op_err!(operand, line_count); Inst::multf()}
                    "divf" | "/f" => {no_op_err!(operand, line_count); Inst::divf()}
    				"shl" | "<<" => {no_op_err!(operand, line_count); Inst::shl()}
    				"shr" | ">>" => {no_op_err!(operand, line_count); Inst::shr()}
    				"and" | "&" => {no_op_err!(operand, line_count); Inst::and()}
    				"or"  | "|" => {no_op_err!(operand, line_count); Inst::or()}
    				"xor" | "^" => {no_op_err!(operand, line_count); Inst::xor()}
    				"not" | "!" => {no_op_err!(operand, line_count); Inst::not()}
                    "jmp" => {
                        match operand.parse::<isize>() {
                            Ok(op) => {
                                Inst::jmp(op)
                            }
                            Err(_) => {
                                jmp_vec.push(&operand);
                                Inst::jmp(-1)
                            }
                        }
                    }

                    "jmpif" | "jif" => {
                        match operand.parse::<isize>() {
                            Ok(op) => {
                                Inst::jmpif(op)
                            }
                            Err(_) => {
                                jmp_vec.push(&operand);
                                Inst::jmpif(-1)
                            }
                        }
                    }

                    "eq" => {no_op_err!(operand, line_count); Inst::eq()}
                    "neq"=> {no_op_err!(operand, line_count); Inst::neq()}
                    "lt" => {no_op_err!(operand, line_count); Inst::lt()}
                    "gt" => {no_op_err!(operand, line_count); Inst::gt()}
                    "eqf"=> {no_op_err!(operand, line_count); Inst::eqf()}
                    "neqf"=>{no_op_err!(operand, line_count); Inst::neqf()}
                    "ltf"=> {no_op_err!(operand, line_count); Inst::ltf()}
                    "gtf"=> {no_op_err!(operand, line_count); Inst::gtf()}
                    "print" | "." => {no_op_err!(operand, line_count); Inst::print()}
                    "shout"=> {no_op_err!(operand, line_count); Inst::shout()}
                    "dump" => {no_op_err!(operand, line_count); Inst::dump()}
                    "empty"=> {no_op_err!(operand, line_count); Inst::empty()}
                    "ifempty"=> {no_op_err!(operand, line_count); Inst::ifempty()}
                    "halt" => {no_op_err!(operand, line_count); Inst::halt()}

                    &_ => {
                        eprintln!("Error: Illegal instruction number: {} or I forgot to include some", inst_vec.len());
                        return Err((ExecErr::IllegalInst, line_count));
                    }
                }
            );
        }

        let mut jmp = 0;
        for i in 0..inst_vec.len() {
            if inst_vec[i].kind.0 == InstType::JMP || inst_vec[i].kind.0 == InstType::JIF {
                if inst_vec[i].operand < 0 {
                    let mut op = 0;
                    for j in 0..label_vec.len() {
                        if label_vec[j].name == jmp_vec[jmp] {
                            op = label_vec[j].addr;
                        }
                    }
                    inst_vec[i].operand = op as isize;
                    jmp += 1;
                }
            }
        }

        let mut constant = 0;
        let mut sub_remain = sub_vec.len();
        for i in 0..inst_vec.len() {
            if !inst_vec[i].kind.1 && inst_vec[i].kind.0 == InstType::PUSH {
                let mut op = 0;
                for j in 0..const_vec.len() {
                    if const_vec[j].name == sub_vec[constant] {
                        op = const_vec[j].value;
                        sub_remain -= 1;
                    }
                }
                inst_vec[i].kind.1 = true;
                inst_vec[i].operand = op as isize;
                constant += 1;
            }
        }

        if sub_remain != 0 {
            eprintln!("Used macro of invalid name");
            return Err((ExecErr::IllegalOperand, 0));
        }

        Ok(inst_vec)
    }
}

/*
// https://stackoverflow.com/questions/27859822/is-it-possible-to-have-stack-allocated-arrays-with-the-size-determined-at-runtim
// would require speed testing
enum StackVec<T, const N: usize> {
    Inline(usize, [T; N]),
    Dynamic(Vec<T>),
}
// */
