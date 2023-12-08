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
    };
}

macro_rules! f64_bool {
    ($dest:expr, $op:tt, $source:expr) => {
        unsafe{ $dest = (transmute::<isize, f64>($dest) $op transmute::<isize, f64>($source)) as isize;}
    };
}

pub mod inst_macro {
    #[macro_export]
    macro_rules! inst {
        ($type:ident) => {
            Inst { kind: (InstType::$type, false), operand: 0}
        };
    }
    #[macro_export]
    macro_rules! inst_op {
        ($type:ident, $op:expr) => {
            Inst { kind: (InstType::$type, true), operand: $op}
        };
    }
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
    pub kind: (InstType, bool),
    pub operand: isize
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
    NEG,
    LT,
    GT,
    LTF,
    GTF,
    PRINT,
    SHOUT,
    DUMP,
    EMPTY,
    IFEMPTY,
    RET,
    FTOI,
    ITOF,
    FLOOR,
    CEIL,
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
    HEX
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
                PrintType::HEX => {
                    for i in 0..self.stack_size-1 {
                        print!("{:X}, ", self.stack[i]);
                    }   println!("{:X}]", self.stack[self.stack_size-1]);
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

            InstType::NEG => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-1] = !(self.stack[self.stack_size-1] > 0) as isize;
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
                unsafe { println!("{i} | {i:X} | {f:.7e}", i=self.stack[self.stack_size-1], f=transmute::<isize, f64>(self.stack[self.stack_size-1])); }
            }

            InstType::SHOUT => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                unsafe { println!("{i} | {i:X} | {f:.7e}", i=self.stack[self.stack_size-1], f=transmute::<isize, f64>(self.stack[self.stack_size-1])); }
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

            InstType::RET => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow);
                }
                self.ip = self.stack[self.stack_size-1] as usize;
                self.stack_size -= 1;
                self.ip += 1;
            }

            InstType::FTOI => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-1] = unsafe {transmute::<isize, f64>(self.stack[self.stack_size-1]) as isize};
            }

            InstType::ITOF => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-1] = unsafe {transmute::<f64, isize>(self.stack[self.stack_size-1] as f64)};
            }

            InstType::FLOOR => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-1] = unsafe {transmute::<f64, isize>(transmute::<isize, f64>(self.stack[self.stack_size-1]).floor())};
            }

            InstType::CEIL => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                }
                self.stack[self.stack_size-1] = unsafe {transmute::<f64, isize>(transmute::<isize, f64>(self.stack[self.stack_size-1]).ceil())};
            }

            InstType::HALT => self.halted = true
        }

        self.ip += 1;
        Ok(())
    }
}

impl Inst {
    pub fn to_string(&self) -> String {
        if self.kind.1 {
            let str = format!("{}", self);
            let (inst, op) = str.split_at(format!("{}", self).find(' ').unwrap());
            let inst = inst.to_lowercase();
            format!("{inst}{op}\n")
        } else {
            format!("{}\n", format!("{}", self).to_lowercase())
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

//const INST_BYTES: usize = size_of::<Inst>()/size_of::<u8>();
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

    pub fn dump_prog_to_file(prog: &Vec<Inst>, dest: &str) -> std::io::Result<()> {
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

        for inst in prog {
            let bytes = unsafe {transmute::<Inst, [u8;16]>(inst.clone())};
            for byte in bytes {
                f_buff.push(byte);
            }
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
        // name, operand, inst number, line
        let mut unchecked_inst_vec: Vec<(&str, &str, isize, usize)> = vec![];
        let mut inst_num: isize = 0;
        let mut label_vec: Vec<Label> = vec![];
        let mut const_vec: Vec<Constant> = vec![];

        for mut line in source.lines() {
            line_count += 1;
            let mut inst = "";
            let mut operand = "";
            let mut char_count = 0;
            let mut comment_count = 0;
            let mut label_count = 0;

            for char in line.chars() {
                if char == ';' || char == '#' {
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
                        if !char.is_ascii() {
                            break;
                        }
                        label_count += 1;
                    }
                    label_vec.push(Label {
                        name: line[char_count-label_count..char_count].to_string(),
                        addr: inst_num as usize
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
            if line.starts_with('%') {
                for char in line.chars() {
                    if char == ' ' {
                        let (const_name, mut value) = line.split_at(char_count);
                        (_,value) = value.split_at(1);
                        const_vec.push(Constant{
                            name: const_name.to_string(),
                            value: if let Ok(v) = value.parse::<isize>() {
                                v
                            } else if let Ok(v) = value.parse::<f64>() {
                                unsafe {transmute::<f64, isize>(v)}
                            } else {
                                eprintln!("Invalid argument in macro definition");
                                return Err((ExecErr::IllegalOperand, line_count));
                            }
                        });
                        line = "";
                        break;
                    }
                    char_count += 1
                }
            }

            if line.trim().len() == 0 {continue;}
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

            unchecked_inst_vec.push((inst, operand, inst_num, line_count));
            inst_num += 1;
        }

        for entry in unchecked_inst_vec {
            let inst = entry.0;
            let operand = entry.1;
            let inst_n = entry.2;
            let line = entry.3;
            inst_vec.push(
                // this could be collapsed a bunch
                match inst {
                    "nop" => {no_op_err!(operand, line); inst!(NOP)}
                    "push" => {
                        if let Ok(op) = operand.parse::<isize>() {
                            inst_op!(PUSH, op)
                        } else if let Ok(op) = operand.parse::<f64>() {
                            inst_op!(PUSH, unsafe {transmute::<f64, isize>(op)})
                        } else if let "$" = operand {
                            inst_op!(PUSH, inst_n)
                        } else {
                            let mut inst = None;
                            for constant in &const_vec {
                                if operand == constant.name {
                                    inst = Some(inst_op!(PUSH, constant.value)); break;
                                }
                            }
                            if let Some(inst) = inst { inst }
                            else { return Err((ExecErr::IllegalOperand, line));}
                        }
                    }

                    "pop" => {no_op_err!(operand, line); inst!(POP)}
                    "dup" => {no_op_err!(operand, line); inst!(DUP)}
                    "pick"=> {no_op_err!(operand, line); inst!(PICK)}
                    "shove"=>{no_op_err!(operand, line); inst!(SHOVE)}
                    "add" | "+" => {no_op_err!(operand, line); inst!(ADD)}
                    "sub" | "-" => {no_op_err!(operand, line); inst!(SUB)}
                    "mult"| "*" => {no_op_err!(operand, line); inst!(MULT)}
                    "div" | "/" => {no_op_err!(operand, line); inst!(DIV)}
                    "addf" | "+f" => {no_op_err!(operand, line); inst!(ADDF)}
                    "subf" | "-f" => {no_op_err!(operand, line); inst!(SUBF)}
                    "multf"| "*f" => {no_op_err!(operand, line); inst!(MULTF)}
                    "divf" | "/f" => {no_op_err!(operand, line); inst!(DIVF)}
    				"shl" | "<<" => {no_op_err!(operand, line); inst!(SHL)}
    				"shr" | ">>" => {no_op_err!(operand, line); inst!(SHR)}
    				"and" | "&" => {no_op_err!(operand, line); inst!(AND)}
    				"or"  | "|" => {no_op_err!(operand, line); inst!(OR)}
    				"xor" | "^" => {no_op_err!(operand, line); inst!(XOR)}
    				"not" | "!" => {no_op_err!(operand, line); inst!(NOT)}
                    "jmp" => {
                        if let Ok(op) = operand.parse::<isize>() {
                            inst_op!(JMP, op)
                        } else {
                            let mut inst = None;
                            for label in &label_vec {
                                if operand == label.name {
                                    inst = Some(inst_op!(JMP, label.addr as isize)); break;
                                }
                            }
                            if let Some(inst) = inst { inst }
                            else { return Err((ExecErr::IllegalAddr, line));}
                        }
                    }

                    "jmpif" | "jif" => {
                        if let Ok(op) = operand.parse::<isize>() {
                            inst_op!(JIF, op)
                        } else {
                            let mut inst = None;
                            for label in &label_vec {
                                if operand == label.name {
                                    inst = Some(inst_op!(JIF, label.addr as isize)); break;
                                }
                            }
                            if let Some(inst) = inst { inst }
                            else { return Err((ExecErr::IllegalAddr, line));}
                        }
                    }

                    "eq" => {no_op_err!(operand, line); inst!(EQ)}
                    "neg"=> {no_op_err!(operand, line); inst!(NEG)}
                    "lt" => {no_op_err!(operand, line); inst!(LT)}
                    "gt" => {no_op_err!(operand, line); inst!(GT)}
                    "ltf"=> {no_op_err!(operand, line); inst!(LTF)}
                    "gtf"=> {no_op_err!(operand, line); inst!(GTF)}
                    "print" | "." => {no_op_err!(operand, line); inst!(PRINT)}
                    "shout"=> {no_op_err!(operand, line); inst!(SHOUT)}
                    "dump" => {no_op_err!(operand, line); inst!(DUMP)}
                    "empty"=> {no_op_err!(operand, line); inst!(EMPTY)}
                    "ifempty"=> {no_op_err!(operand, line); inst!(IFEMPTY)}
                    "ret"  => {no_op_err!(operand, line); inst!(RET)}
                    "ftoi" => {no_op_err!(operand, line);
                        if inst_vec[inst_n as usize-1].clone() != inst!(CEIL)
                        && inst_vec[inst_n as usize-1].clone() != inst!(FLOOR) {
                            eprintln!("WANRING: It is recomended to use 'ceil' or 'floor' before casting to integer");
                        } // for some reason it doesn't work w/out clone
                        inst!(FTOI)
                    }
                    "itof" => {no_op_err!(operand, line); inst!(ITOF)}
                    "floor"=> {no_op_err!(operand, line); inst!(FLOOR)}
                    "ceil" => {no_op_err!(operand, line); inst!(CEIL)}
                    "halt" => {no_op_err!(operand, line); inst!(HALT)}

                    &_ => {
                        eprintln!("Error: Illegal instruction number: {} or I forgot to include some", inst_vec.len());
                        return Err((ExecErr::IllegalInst, line));
                    }
                }
            );
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
