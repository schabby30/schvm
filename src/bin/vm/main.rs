use std::path::PathBuf;

use schvm::vm::{Machine, RegisterName};
use schvm::memory::Addressable;
use schvm::instructions::Instruction;
use schvm::lexer::compile;

pub fn main() -> Result<(), String> {
    let mut machine = Machine::new();
    machine.set_register(RegisterName::PC, 0);

    let file_path = PathBuf::from("src/bin/vm/source.asm");

    let bytes = compile(file_path)?;

    let mut counter = 0;
    for byte in &bytes {
        let _ = machine.memory.write_byte(counter, *byte);
        counter += 1;
    }

    println!("bytes: {:?}", bytes);

    loop {
        /* println!("\n***   STEP   ***");
        println!("Registers: {:?}", machine.get_register_list());
        println!("Memory: {:?}", &machine.memory[0..22]); */
        
        let pc = machine.get_register(RegisterName::PC);
        let read_byte = machine.memory.read_byte(pc)?;

        match Instruction::handle_instruction(&mut machine, read_byte, pc) {
            Ok(()) => {},
            Err(e) => match e.as_str() {
                "Unknown instruction" => panic!("End of World... : {:?}", e),
                _ => break,
            },
        }
    }

    println!("Registers: {:?}", machine.get_register_list());
    println!("Memory: {:?}", &machine.memory[0..19]);

    Ok(())
}