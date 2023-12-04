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

Example of how to use different floating point operations and debug flag.
```sh
cargo build -r
./src/examples/float.sh
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
eq          ;check if two values at the top are equal substitutes them with the result
neq         ;check if two values are not equal
gt          ;check if the value below is greater than the one on top
lt          ;opposite of gt
empty       ;empties the stack
ifempty     ;if stack is empty pushes true, false if not
print       ;or . - print the value at the top of the stack (doesn't pop it)
shout       ;prints and pops
dump        ;prints the entire stack
halt        ;stops the execution
```

TODO:
- [x] implement the instruction set
- [x] implement the assembly
- [x] imlpement jump labels
- [x] make readme useful
- [x] make a disassembler (easy)
- [x] add compile time value definitions
- [x] make compiler errors give line, not instruction, numbers
- [x] make floats work
- [ ] add access functions of vm instead of pub
- [ ] add heap w/ pointers on the stack
- [ ] add comments in some places
- [x] compiler might not give errors on pushing not defined constants/labels
- [ ] make two type of instruction struct to save space on unused operands (indicate if contains operand to make file parsing possible)
- [ ] use from_bites for vector conversion
- [ ] (technical regarding implementation) use f64::from_bites and the like, keep track of changes in implementation in respect to std::mem::trasmute
- [ ] some refactoring regarding instruction storage/parsing https://www.youtube.com/watch?v=LN9vrbBNG64 ~1:50:00
- [ ] implement the language
- might change the executable names
