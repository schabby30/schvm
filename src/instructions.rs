use crate::vm::{Machine, RegisterName};
use crate::memory::Addressable;

pub enum Instruction {
    Load = 0b00000001, // load a value to Register:A
    Move = 0b00000010, // move the value in Register:A to a memory address given in the argument
    Halt = 0b10000000, // halt the execution
}

impl Instruction {
    pub fn handle_instruction(machine: &mut Machine, instruction: u8, pc: u16) -> Result<(), String>{
        match instruction {
            i if i == Instruction::Load as u8 => {
                machine.set_register(RegisterName::A, machine.memory.read_word(pc + 1)?);
                machine.set_register(RegisterName::PC, pc + 3);
            },
            i if i == Instruction::Move as u8 => {

            },
            i if i == Instruction::Halt as u8 => return Err("Halt".to_string()),
            _ => return Err("Unknown instruction".to_string()),
        }

        Ok(())
    }
}