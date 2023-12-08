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
```sh
cargo build -r
./src/examples/fib_term.sh
./src/examples/float.sh
./src/examples/bitwise.sh
./src/examples/gray.sh
./src/examples/euler.sh
./src/examples/variadics.sh
```

## Writing programs
Comments can be either ; or # to allow for the use of C preprocessor
``` nasm
start:      ;create a label
nop         ;no op
push 7      ;push 7 on the stack
pop         ;pop the top of the stack
dup         ;duplicate top of the stack
pick        ;gets top value and replaces it with one that is that amount lower in the stack
shove       ;gets two top values and pushes the 1 one lower on the stack by the amount specified by the second
add         ;add two values at the top of the stack
sub         ;subtract two values at the top of the stack
mult        ;multiply two values at the top of the stack
div         ;divide two values at the top of the stack
shl         ;perform shift left on second top value, top of stack, amount of times
shr         ;perform shift right
and         ;perform bitwise and on two top values
or          ;perform bitwise or on two top values
xor         ;perform bitwise xor on two top values
not         ;perform bitwise not on two top values
jmp 10      ;jump to instruction numer 10 (0 based)
jmpif 2     ;or jif - jump to instruction numer 2 if check was true
jmp start   ;jmp to 'start' label
eq          ;check if two values at the top are equal substitutes them with the result
neq         ;check if two values are not equal
gt          ;check if the value below is greater than the one on top
lt          ;opposite of gt
print       ;or . - print the value at the top of the stack (doesn't pop it)
shout       ;prints and pops
dump        ;prints the entire stack
empty       ;empties the stack
ifempty     ;if stack is empty pushes true, false if not
ret         ;return from subroutine (accounts for return instruction offset, see examples/implementation)
ftoi        ;convert value from float to integer
itof        ;convert value from integer to float
floor       ;floor float
ceil        ;ceil float
halt        ;stops the execution
```

## TODO
- [x] implement the instruction set
- [x] implement the assembly
- [x] imlpement jump labels
- [x] make readme useful
- [x] make a disassembler (easy)
- [x] add compile time value definitions
- [x] make compiler errors give line, not instruction, numbers
- [x] make floats work
- [x] add bitwise instructions
- [x] add subroutines
- [ ] add access functions of vm instead of pub
- [ ] add heap w/ pointers on the stack - predefined native inst (ex. malloc) + including linux/windows ...
- [ ] add comments in some places
- [x] use macros to reduce number of lines (there are technically still some)
- [x] labels don't parse '_'
- [ ] add more tests
- [x] add --help
- [x] compiler might not give errors on pushing not defined constants/labels
- [x] make push the only inst w/ operand
- [x] do label/const substitution while parsing instructions
- [ ] crash on redefined labels
- [ ] include files https://www.youtube.com/watch?v=k6qk6lT4S3U ~2:00:00+
- [ ] use split_whitespace() while parsing
- [x] use from_bites for vector conversion
- [ ] some refactoring regarding instruction storage/parsing https://www.youtube.com/watch?v=LN9vrbBNG64 ~1:50:00
- [ ] reduce inst size (dynamic interpretation)
- [ ] bake bytecode dynamic
- [ ] experiment with stack allocated inst vec
- [ ] implement the language
- might change the executable names
