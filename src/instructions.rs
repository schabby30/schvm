use crate::vm::{Machine, RegisterName};
use crate::memory::Addressable;

pub enum Instruction {
    LoadA = 0b00000001, // load a value to Register:A
    LoadB = 0b00000010, // load a value to Register:A
    LoadC = 0b00000011, // load a value to Register:A
    Move = 0b00000100, // move the value in Register:A to a memory address given in the argument
    Add = 0b00001000, // add Register:B and Register:C and store the result in Register:A
    Jump = 0b00010000, // jump to the address given in the argument
    Halt = 0b10000000, // halt the execution
}

impl Instruction {
    pub fn handle_instruction(machine: &mut Machine, instruction: u8, pc: u16) -> Result<(), String>{
        match instruction {
            i if i == Instruction::LoadA as u8 => {
                machine.set_register(RegisterName::A, machine.memory.read_word(pc + 1)?);
                machine.set_register(RegisterName::PC, pc + 3);
            },
            i if i == Instruction::LoadB as u8 => {
                machine.set_register(RegisterName::B, machine.memory.read_word(pc + 1)?);
                machine.set_register(RegisterName::PC, pc + 3);
            },
            i if i == Instruction::LoadC as u8 => {
                machine.set_register(RegisterName::C, machine.memory.read_word(pc + 1)?);
                machine.set_register(RegisterName::PC, pc + 3);
            },
            i if i == Instruction::Move as u8 => {
                let address = machine.memory.read_word(pc + 1)?;
                let value = machine.get_register(RegisterName::A);
                machine.memory.write_word(address, value)?;
                machine.set_register(RegisterName::PC, pc + 3);
            },
            i if i == Instruction::Add as u8 => {
                let reg_b = machine.get_register(RegisterName::B);
                let reg_c = machine.get_register(RegisterName::C);
                machine.set_register(RegisterName::A, reg_b + reg_c);
                machine.set_register(RegisterName::PC, pc + 1);
            },
            i if i == Instruction::Jump as u8 => {
                let address = machine.memory.read_word(pc + 1)?;
                machine.set_register(RegisterName::PC, address);
            },
            i if i == Instruction::Halt as u8 => return Err("Halt".to_string()),
            _ => return Err("Unknown instruction".to_string()),
        }

        Ok(())
    }
}