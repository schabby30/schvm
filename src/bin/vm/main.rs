use schvm::vm::{Machine, RegisterName};
use schvm::memory::Addressable;
use schvm::instructions::Instruction;

pub fn main() -> Result<(), String> {
    let mut machine = Machine::new();
    machine.set_register(RegisterName::A, 13);
    machine.set_register(RegisterName::PC, 0);
    let _ = machine.memory.write_word(0, 511);
    let _ = machine.memory.write_byte(3, 128);

    loop {
        println!("{:?}", machine.get_register_list());
        println!("{:?}", &machine.memory[0..10]);
        
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

    Ok(())
}