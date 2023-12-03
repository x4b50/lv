# LV - Lada Virtual
A simple stack-based virtual машина (machine) with a custom assembly compiler and no external dependencies.

## Using the VM
Compiling the program, running the compiler and the virtual machine with stack size equal 32 (default) in debug mode.

``` sh
cargo build -r
./lc code.lv code.lb
./lv code.lb 32 -d
```

You can also disassemble the binary.
```sh
./ldis code.lb
```

## Running examples
The fibonacci example will cause stack overflow, because it has no boundary checks. It is not a bug.
```sh
cargo build -r
./src/examples/fib.sh
```

For an example of how to use conditions run fib_term example.
```sh
cargo build -r
./src/examples/fib_term.sh
```

## Writing programs
``` nasm
start:      ;create a label
nop         ;no op
push 7      ;push 7 on the stack
pop         ;pop the top of the stack
dup         ;duplicate top of the stack
pick 2      ;copy second value from the top of the stack - pick 0 = dup
add         ;add two values at the top of the stack
sub         ;subtract two values at the top of the stack
mult        ;multiply two values at the top of the stack
div         ;divide two values at the top of the stack
jmp 10      ;jump to instruction numer 10 (0 based)
jmpif 2     ;or jif - jump to instruction numer 2 if check was true
jmp start   ;jmp to 'start' label
eq          ;check if two values at the top are equal and pushes the result on the stack
neq         ;check if two values are not equal
gt          ;check if the value below is greater than the one on top
lt          ;opposite of gt
print       ;or . - print the value at the top of the stack (doesn't pop it)
dump        ;prints the entire stack
halt        ;stops the execution
```

TODO:
- [x] implement the instruction set
- [x] implement the assembly
- [x] imlpement jump labels
- [x] make readme useful
- [x] make a disassembler (easy)
- [ ] add compile time value definitions
- [ ] add heap w/ pointers on the stack
- [ ] implement the language
- [ ] add comments in some places
- might change the executable names
