# Simple Assembler - Compiler - VM in Rust

### Prerequisites:

Install Rust and Cargo

https://doc.rust-lang.org/cargo/getting-started/installation.html


### Usage:

> After cloning the repo:

```
cd ./schvm
cargo build
cargo run
```


### Write your own assembly code:

> You can write your own assembly program by editing the **./src/bin/vm/source.asm** file


### Currently supported assembly instructions:
```
lda value      :  load a value to Register:A
ldb value      :  load a value to Register:A
ldc value      :  load a value to Register:A
sta address    :  store a value at given address to Register:A
stb address    :  store a value at given address to Register:B
stc address    :  store a value at given address to Register:C
mov address    :  move the value in Register:A to a given memory address
add            :  add Register:B and Register:C and store the result in Register:A
jmp address    :  jump to the address given in the argument
Jio address    :  jump to the address given in the argument if overflow flag is 1
halt           :  halt the execution
```
