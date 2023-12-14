# LV - Lada Virtual
A simple stack-based virtual машина (machine) with a custom assembly compiler and no external dependencies.

## Using the VM
Compiling the program, running the compiler and the virtual machine with stack size equal 32 (default) in debug mode.

``` sh
cargo build -r
./lc code.lv code.lb
./lv code.lb -s 32 -d
```

You can also disassemble the binary.
```sh
./ldis code.lb
```

Getting help inormation
```sh
./lv --help
```

## Running examples
(Some examples are not implementes optimaly, because of later addition of more fitting instructions)
```sh
cargo build -r
./src/examples/hello_world.sh
./src/examples/fib_term.sh
./src/examples/float.sh
./src/examples/bitwise.sh
./src/examples/gray.sh
./src/examples/euler.sh
./src/examples/variadics.sh
./src/examples/arena.sh
./src/examples/native.sh
./src/examples/native_malloc.sh
./src/examples/resize.sh
```

## Writing programs
Comments can be either ; or # to allow for the use of C preprocessor
``` nasm
start:      ;create a label
halt        ;stops the execution
nop         ;no op
push 7      ;push 7 on the stack
pop         ;pop the top of the stack
dup         ;duplicate top of the stack
swap        ;consume top value and swap second with one that is <top> values below
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
not         ;perform bitwise not on the top value
jmp 10      ;jump to instruction numer 10 (0 based)
jmpif 2     ;or jif - jump to instruction numer 2 if check was true
jmp start   ;jmp to 'start' label
eq          ;check if two values at the top are equal substitutes them with the result
gt          ;check if the value below is greater than the one on top
lt          ;check if less than
neg         ;negate the bool value
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
write8      ;write 8 lowest bits from second to last value on arena adress sepcified by top of the stack
write16     ;same but 16 bits
write32     ;same but 32 bits
write64     ;same but 64 bits
read8       ;read 8 bits from arena adress specified by top of the stack
read16      ;same but 16 bits
read32      ;same but 32 bits
read64      ;same but 64 bits
native      ;calls native function with the index at the top of the stack
```

## TODO
Make all the below issues, or something else that makes sense
- [ ] do some clean up
- [ ] add versioning system for byte code
- [ ] explain the bit shenanigans
- [ ] add comments in some places
- [ ] add more tests
- [ ] include files https://www.youtube.com/watch?v=k6qk6lT4S3U ~2:00:00+
- [ ] use split_whitespace() while parsing
- [ ] make arena static and possibly stack allocated
- [ ] experiment with stack allocated inst vec
- [ ] create some docs for the assembly
