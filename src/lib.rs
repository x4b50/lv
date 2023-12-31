// #[cfg(target_os = "linux")]
pub mod linux;
#[cfg(test)]
mod tests;
use core::fmt;
use std::{mem::transmute, isize};

const PTR_OFFSET: usize = 48;
const PTR_MASK: isize = 0x0000ffffffffffff;

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

macro_rules! mem_check {
    ($self:ident, $type_len:tt, $mem:ident, $index:ident) => {
        if $self.stack[$self.stack_size-1] >= (1<<PTR_OFFSET) {
            let dyn_mem = &mut $self.dyn_mem[($self.stack[$self.stack_size-1]>>PTR_OFFSET)as usize-1];
            $mem = if let Some(m) = dyn_mem {
                if ($self.stack[$self.stack_size-1]&PTR_MASK) +$type_len > m.len() as isize {return Err(ExecErr::IllegalMemAccess); }
                $index = 0;
                Some(m)
            } else { return Err(ExecErr::IllegalMemAccess); }
        }
        if $mem == None {
            if $self.stack[$self.stack_size-1]+$type_len > $self.arena.len() as isize {
                return Err(ExecErr::IllegalMemAccess);
            }
            $index = $self.stack[$self.stack_size-1];
            $mem = Some(&mut $self.arena);
        }
        if $index < 0 { return Err(ExecErr::NativeError); }
    };
}

macro_rules! read_mem {
    ($self:ident, $type_len:tt, $type:tt) => {
        let mut mem: Option<&Vec<u8>> = None;
        let mut index: isize = -1;
        mem_check!($self, $type_len, mem, index);
        let index = index as usize;
        let bytes: &[u8; $type_len] = if let Some(m) = mem { match
            m[index..index+$type_len].try_into() {Ok(v)=>{v}
            Err(_)=>{unreachable!()}}
        } else {return Err(ExecErr::IllegalMemAccess);};
        $self.stack[$self.stack_size-1] = $type::from_ne_bytes(*bytes) as isize;
    };
}

macro_rules! write_mem {
    ($self:ident, $type_len:tt, $type:tt) => {
        let mut mem: Option<&mut Vec<u8>> = None;
        let mut index: isize = -1;
        mem_check!($self, $type_len, mem, index);
        let index = index as usize;
        if let Some(m) = mem {
            let bytes = ($self.stack[$self.stack_size-2] as $type).to_ne_bytes();
            for i in 0..bytes.len() {m[index+i] = bytes[i]}
            $self.stack_size -= 2;
        }
    };
}

pub mod inst_macro {
    #[macro_export]
    macro_rules! inst {
        ($type:ident) => {
            Inst { kind: InstType::$type, has_op: false, operand: 0}
        };
    }
    #[macro_export]
    macro_rules! inst_op {
        ($type:ident, $op:expr) => {
            Inst { kind: InstType::$type, has_op: true, operand: $op}
        };
    }
}

pub struct Lada {
    halted: bool,
    ip: usize,
    stack_size: usize,
    stack: Vec<isize>,
    arena: Vec<u8>,
    program: Vec<Inst>,
    dyn_mem: Vec<Option<Vec<u8>>>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub inst: Vec<Inst>,
    pub mem: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inst {
    pub kind: InstType,
    pub has_op: bool,
    pub operand: isize
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstType {
    HALT,
    NOP,
    PUSH,
    POP,
    DUP,
    SWAP,
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
    JMP,
    JIF,
    EQ,
    NEG,
    LT,
    GT,
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
    READ_8,
    READ_16,
    READ_32,
    READ_64,
    WRITE_8,
    WRITE_16,
    WRITE_32,
    WRITE_64,
    NATIVE,
    MALLOC,
    FREE,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExecErr {
    StackOverflow,
    StackUnderflow,
    IllegalInst,
    DivByZero,
    NoOperand,
    IllegalAddr,
    IllegalInstAddr,
    IllegalOperand,
    Redefinition,
    IllegalMemAccess,
    NativeError,
}

pub enum PrintType {
    I64,
    F64,
    HEX
}

type Native = fn(&mut Lada) -> Result<(), ExecErr>;

impl Lada {
    pub fn init(program: Program, stack_cap: usize, arena_size: usize) -> Lada {
        let mut arena = program.mem;
        if arena_size > arena.len() {arena.resize(arena_size, 0);}
        Lada {
            halted: false,
            ip: 0,
            stack_size: 0,
            stack: vec![0; stack_cap],
            arena,
            program: program.inst,
            // because 0<<48 == zero chunk addresses will be offset by 1
            dyn_mem: vec![],
        }
    }

    pub fn ip(&self) -> usize {self.ip}
    pub fn halted(&self) -> bool {self.halted}
    pub fn inst(&self, n: usize) -> &Inst {&self.program[n]}
    pub fn prog_len(&self) -> usize {self.program.len()}
    pub fn stack_extend(&mut self, n: usize) { self.stack.resize(self.stack.len()+n, 0); }
    pub fn get_arena(&self) -> &[u8] {&self.arena}
    pub fn resize_arena(&mut self, n: usize) { self.arena.resize(n, 0); }
    pub fn last_err_inst(&self) -> &InstType { &self.program[self.ip].kind }
    pub fn get_stack_top(&self, n: usize) -> &[isize] { &self.stack[self.stack_size-n..self.stack_size] }
    pub fn get_dyn_mem(&self) -> &[Option<Vec<u8>>] {&self.dyn_mem}

    pub fn print_stack(&self, t: &PrintType) {
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
        match inst.kind {
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

            InstType::SWAP => {
                if self.stack[self.stack_size-1] < 0 || self.stack[self.stack_size-1] >= self.stack_size as isize {
                    return Err(ExecErr::IllegalAddr);
                }
                self.stack_size -=1;
                let adr = self.stack[self.stack_size];
                let tmp = self.stack[self.stack_size-1];
                self.stack[self.stack_size-1] = self.stack[self.stack_size -1 -adr as usize];
                self.stack[self.stack_size -1 -adr as usize] = tmp;
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
                print!("Stack: ");
                self.print_stack(&print_type);
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

            InstType::READ_8 => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                } read_mem!(self, 1, u8);
            }
            InstType::READ_16 => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                } read_mem!(self, 2, u16);
            }
            InstType::READ_32 => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                } read_mem!(self, 4, u32);
            }
            InstType::READ_64 => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow)
                } read_mem!(self, 8, u64);
            }

            InstType::WRITE_8 => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                } write_mem!(self, 1, u8);
            }
            InstType::WRITE_16 => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                } write_mem!(self, 2, u16);
            }
            InstType::WRITE_32 => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                } write_mem!(self, 4, u32);
            }
            InstType::WRITE_64 => {
                if self.stack_size < 2 {
                    return Err(ExecErr::StackUnderflow)
                } write_mem!(self, 8, u64);
            }

            InstType::NATIVE => {
                if self.stack_size < 1 {
                    return Err(ExecErr::StackUnderflow);
                }
                self.stack_size -= 1;
                match self.native(self.stack[self.stack_size] as usize) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                }
            }

            InstType::MALLOC => {
                let mut found = false;
                let mut adr: isize = -1;
                for i in 0..self.dyn_mem.len() {
                    if self.dyn_mem[i] == None {
                        self.dyn_mem[i] = Some(vec![0;self.stack[self.stack_size-1]as usize]);
                        found = true;
                        adr = (i+1 << PTR_OFFSET)as isize;
                        break
                    }
                }
                if !found {
                    adr = (self.dyn_mem.len()+1 << PTR_OFFSET)as isize;
                    self.dyn_mem.push(Some(vec![0;self.stack[self.stack_size-1]as usize]));
                }
                if adr < 0 { return Err(ExecErr::NativeError); }
                self.stack[self.stack_size-1] = adr;
            }

            InstType::FREE => {
                self.stack_size -= 1;
                let adr = (self.stack[self.stack_size] >> PTR_OFFSET) as usize-1;
                self.dyn_mem[adr] = None;
            }
            InstType::HALT => self.halted = true
        }
        self.ip += 1;
        Ok(())
    }
}

// todo: look up something about DebugStruct and alike
impl fmt::Debug for Lada {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Lada {{ ")?;
        write!(f, "halted: {}, ", self.halted)?;
        write!(f, "ip: {}\n", self.ip)?;
        write!(f, "program: [\n")?;
        for inst in &self.program {
            write!(f, " {{ {inst} }}")?;
        }
        write!(f, " ]\n")?;
        write!(f, "stack size: {}\n", self.stack_size)?;
        write!(f, "stack used: ")?;
        self.print_stack(&PrintType::I64);
        write!(f, "stack full: {:?}\n", self.stack)?;
        write!(f, "arena: {:?}\n", self.arena)?;
        write!(f, "dynamic memory: {:?}", self.dyn_mem)?;
        write!(f, " }}")?;
        Ok(())
    }
}

impl Inst {
    pub fn to_string(&self) -> String {
        if self.has_op {
            let str = format!("{}", self);
            let (inst, op) = str.split_at(format!("{}", self).find(' ').unwrap());
            let inst = inst.to_lowercase();
            format!("{inst}{op}")
        } else {
            format!("{}", format!("{}", self).to_lowercase())
        }
    }
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.has_op {
            write!(f, "{:?} {}", self.kind, self.operand)
        } else {
            write!(f, "{:?}", self.kind)
        }
    }
}

pub mod file {
    use std::{fs, mem::size_of};
    use super::*;

    pub fn read_prog_from_file(source: &str) -> std::io::Result<Program> {
        assert!(size_of::<InstType>() == size_of::<u8>(), "InstType is no longer 8bits long");
        let buff = fs::read(source)?;
        let mut prog: Program = Program { inst: vec![], mem: vec![] };

        let len = usize::from_ne_bytes(buff[..8].try_into().unwrap());
        let mut i = 8+len;
        for b in 8..i {
            prog.mem.push(buff[b]);
        }

        while i < buff.len() {
            let mut operand = None;
            let inst_type: InstType = unsafe {transmute(buff[i])};
            i += 1;

            if let InstType::PUSH | InstType::JMP | InstType::JIF = inst_type {
                assert!(i+7 < buff.len(), "Corrupted file");
                operand = Some(isize::from_ne_bytes(match buff[i..i+8].try_into() {
                    Ok(v) => {v}
                    Err(_) => {unreachable!()}
                }));
                i += 8;
            }

            match operand {
                None =>     prog.inst.push(Inst { kind: inst_type, has_op: false, operand: 0 }),
                Some(op) => prog.inst.push(Inst { kind: inst_type, has_op: true, operand: op })
            }
        }
        Ok(prog)
    }

    pub fn dump_prog_to_file(prog: &Program, dest: &str) -> std::io::Result<()> {
        assert!(size_of::<InstType>() == size_of::<u8>(), "InstType is no longer 8bits long");
        std::fs::File::create(dest)?;
        match fs::OpenOptions::new().write(true).open(dest) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error opening file {e}");
                return Err(e);
            }
        }

        let mut f_buff: Vec<u8> = vec![];
        f_buff.extend(usize::to_ne_bytes(prog.mem.len()).iter());
        f_buff.extend(prog.mem.iter());
        for inst in &prog.inst {
            let byte: &u8 = unsafe {transmute(&inst.kind)};
            f_buff.push(*byte);

            if inst.has_op {
                for byte in inst.operand.to_ne_bytes() {
                    f_buff.push(byte);
                }
            }
        }

        match fs::write(dest, &f_buff) {
            Ok(_) => {Ok(())}
            Err(e) => {
                eprintln!("Error writing to a file {dest}: {e}");
                return Err(e);
            }
        }
    }

    #[derive(Debug)]
    struct Label<'a> {
        name: &'a str,
        addr: usize
    }

    #[derive(Debug)]
    struct Constant<'a> {
        name: &'a str,
        value: isize
    }

    // will have to change or it will become a piece of spaghetti
    pub fn asm_parse(source: &str) -> Result<Program, (ExecErr, usize)> {
        let mut line_count = 0;
        let mut inst_vec: Vec<Inst> = vec![];
        let mut mem: Vec<u8> = vec![];
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
                    let label = Label {
                        name: &line[char_count-label_count..char_count],
                        addr: inst_num as usize
                    };

                    for lbl in &label_vec {
                        if lbl.name == label.name {
                            eprintln!("Redefined label");
                            return Err((ExecErr::Redefinition, line_count));
                        }
                    }
                    label_vec.push(label);
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
            if line.starts_with('%') || line.starts_with('@') {
                for char in line.chars() {
                    if char == ' ' {
                        let (const_name, mut value) = line.split_at(char_count);
                        (_,value) = value.split_at(1);
                        let constant = Constant{
                            name: const_name,
                            value: if line.starts_with('@') && value.starts_with('"') {
                                let str = value.trim_matches('"').replace("\\n", "\n").replace("\\t", "\t").replace("\\0", "\0");
                                let bytes = str.as_bytes();
                                let adr = mem.len();
                                mem.extend(bytes);
                                adr as isize
                            } else if let Ok(v) = value.parse::<isize>() {
                                if line.starts_with('@') {let adr = mem.len()as isize; mem.extend(v.to_ne_bytes()); adr}
                                else {v}
                            } else if let Ok(v) = value.parse::<usize>() {
                                if line.starts_with('@') {let adr = mem.len()as isize; mem.extend(v.to_ne_bytes()); adr}
                                else {v as isize}
                            } else if let Ok(v) = isize::from_str_radix(value.trim_start_matches("0x"), 16) {
                                if line.starts_with('@') {let adr = mem.len()as isize; mem.extend(v.to_ne_bytes()); adr}
                                else {v}
                            } else if let Ok(v) = usize::from_str_radix(value.trim_start_matches("0x"), 16) {
                                if line.starts_with('@') {let adr = mem.len()as isize; mem.extend(v.to_ne_bytes()); adr}
                                else {v as isize}
                            } else if let Ok(v) = value.parse::<f64>() {
                                if line.starts_with('@') {let adr = mem.len()as isize; mem.extend(v.to_ne_bytes()); adr}
                                else {unsafe {transmute::<f64, isize>(v)}}
                            } else {
                                eprintln!("Invalid argument in macro definition");
                                return Err((ExecErr::IllegalOperand, line_count));
                            }
                        };

                        for cst in &const_vec {
                            if cst.name == constant.name {
                                eprintln!("Redefined constant");
                                return Err((ExecErr::Redefinition, line_count));
                            }
                        }
                        const_vec.push(constant);
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
            let operand = entry.1;
            let inst_n = entry.2;
            let line = entry.3;
            // this could be collapsed a bunch
            // todo: with a macro create an associated array/hashmap of "name" -> InstType
            inst_vec.push(
                match entry.0 {
                    "nop" => {no_op_err!(operand, line); inst!(NOP)}
                    "push" => {
                        if let Ok(op) = operand.parse::<isize>() {
                            inst_op!(PUSH, op)
                        } else if let Ok(op) = isize::from_str_radix(operand.trim_start_matches("0x"), 16) {
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
                    "swap"=> {no_op_err!(operand, line); inst!(SWAP)}
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
                    "print" | "." => {no_op_err!(operand, line); inst!(PRINT)}
                    "shout"=> {no_op_err!(operand, line); inst!(SHOUT)}
                    "dump" => {no_op_err!(operand, line); inst!(DUMP)}
                    "empty"=> {no_op_err!(operand, line); inst!(EMPTY)}
                    "ifempty"=> {no_op_err!(operand, line); inst!(IFEMPTY)}
                    "ret"  => {no_op_err!(operand, line); inst!(RET)}
                    "ftoi" => {no_op_err!(operand, line);
                        if inst_vec[inst_n as usize-1] != inst!(CEIL)
                        && inst_vec[inst_n as usize-1] != inst!(FLOOR) {
                            eprintln!("WANRING: It is recomended to use 'ceil' or 'floor' before casting to integer");
                        } inst!(FTOI)
                    }
                    "itof" => {no_op_err!(operand, line); inst!(ITOF)}
                    "floor"=> {no_op_err!(operand, line); inst!(FLOOR)}
                    "ceil" => {no_op_err!(operand, line); inst!(CEIL)}
                    "read8" => {no_op_err!(operand, line); inst!(READ_8)}
                    "read16" => {no_op_err!(operand, line); inst!(READ_16)}
                    "read32" => {no_op_err!(operand, line); inst!(READ_32)}
                    "read64" => {no_op_err!(operand, line); inst!(READ_64)}
                    "write8" => {no_op_err!(operand, line); inst!(WRITE_8)}
                    "write16"=> {no_op_err!(operand, line); inst!(WRITE_16)}
                    "write32"=> {no_op_err!(operand, line); inst!(WRITE_32)}
                    "write64"=> {no_op_err!(operand, line); inst!(WRITE_64)}
                    "native" => {no_op_err!(operand, line); inst!(NATIVE)}
                    "malloc" => {no_op_err!(operand, line); inst!(MALLOC)}
                    "free"   => {no_op_err!(operand, line); inst!(FREE)}
                    "halt" => {no_op_err!(operand, line); inst!(HALT)}
                    &_ => {
                        eprintln!("Error: Illegal instruction number: {} or I forgot to include some", inst_vec.len());
                        return Err((ExecErr::IllegalInst, line));
                    }
                }
            );
        }

        Ok(Program { inst: inst_vec, mem })
    }
}
/* https://stackoverflow.com/questions/27859822/is-it-possible-to-have-stack-allocated-arrays-with-the-size-determined-at-runtim  -  would require speed testing
enum StackVec<T, const N: usize> {
    Inline(usize, [T; N]),
    Dynamic(Vec<T>),
} // */
