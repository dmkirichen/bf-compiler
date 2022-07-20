// This is my own implementation of Brainfuck interpretator/compilator,
// written in Rust programming language
//
// 19.07.2022
//
// Basic Usage of Brainfuck:
//   > = increases memory pointer, or moves the pointer to the right 1 block.
//   < = decreases memory pointer, or moves the pointer to the left 1 block.
//   + = increases value stored at the block pointed to by the memory pointer
//   - = decreases value stored at the block pointed to by the memory pointer
//   [ = like c while(cur_block_value != 0) loop.
//   ] = if block currently pointed to's value is not zero, jump back to [
//   , = like c getchar(). input 1 character.
//   . = like c putchar(). print 1 character to the console
//
// Rules:
//   - any arbitrary character besides the 8 listed above should be ignored 
//     by the compiler or interpretor. Characters besides the 8 operators should be 
//     considered comments.
//   - all memory blocks on the "array" are set to zero at the beginning of the program, 
//     and the memory pointer starts out on the very left most memory block.
//   - loops may be nested as many times as you want. But all [ must have a corresponding ].

use std::io::Read;

fn main() {
    const NUM_BLOCKS: usize = 300;  // number of blocks in bf
    let mut blocks: [i32; NUM_BLOCKS] = [0; NUM_BLOCKS];  // fixed size array is enough
    let mut mem_p = 0;  // memory pointer
    let command = "Hello World! 
                   >+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.
                   +++++++..+++.>>>++++++++[<++++>-]<.>>>++++++++++[<+++++++++>-]<---
                   .<<<<.+++.------.--------.>>+.>++++++++++.";
    let mut command_p = 0;
    let mut brackets_stack = vec![];
    while command_p < command.chars().count() {
        let c = command.chars().nth(command_p);  // char at 'command_p' place of command
        if c == Some('>') {
            mem_p += 1;
        } else if c == Some('<') {
            mem_p -= 1;
        } else if c == Some('+') {
            blocks[mem_p] += 1;   
        } else if c == Some('-') {
            if blocks[mem_p] > 0 {
                blocks[mem_p] -= 1;
            }
        } else if c == Some('[') {
            brackets_stack.push(command_p);
        } else if c == Some(']') {
            if blocks[mem_p] > 0 {
                command_p = brackets_stack.pop().expect("There was no corresponding '[' bracket for this ']'.");
                continue;
            }
        } else if c == Some(',') {
            // input 1 char from a console
            let input: Option<i32> = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as i32);
            blocks[mem_p] = input.expect("No valid byte was entered.");
        } else if c == Some('.') {
            println!("{}", blocks[mem_p]);
        }

        command_p += 1;
    }

    println!("\nFinal state of memory blocks:\n{:?}", blocks);
}
