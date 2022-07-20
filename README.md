# Brainfuck Compiler

***Notice: The code will be gradually improved, which is very convenient fact, because now you don't know if I am just bad at programming or working slowly on the code.***

This is my own implementation of Brainfuck interpretator/compilator,
written in Rust programming language

### Basic Usage

  - `>` increases memory pointer, or moves the pointer to the right 1 block.
  - `<` decreases memory pointer, or moves the pointer to the left 1 block.
  - `+` increases value stored at the block pointed to by the memory pointer
  - `-` decreases value stored at the block pointed to by the memory pointer
  - `[` like c while(cur_block_value != 0) loop.
  - `]` if current block value is not zero, jump back to [
  - `,` inputs 1 character from the console
  - `.` prints 1 character to the console

### Rules
  - any arbitrary character besides the 8 listed above should be ignored 
    by the compiler or interpretor. Characters besides the 8 operators should be 
    considered comments.
  - all memory blocks on the "array" are set to zero at the beginning of the program, 
    and the memory pointer starts out on the very left most memory block.
  - loops may be nested as many times as you want. But all `[` must have a corresponding `]`.

