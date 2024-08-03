use crate::vm::{Flags, Machine, RegisterName};
use crate::memory::Addressable;

pub enum Instruction {
    LoadA = 0b00000001, // load a value to Register:A
    LoadB = 0b00000010, // load a value to Register:A
    LoadC = 0b00000011, // load a value to Register:A
    StoreA = 0b00000100, // store a value at given address to Register:A
    StoreB = 0b00001000, // store a value at given address to Register:B
    StoreC = 0b00001100, // store a value at given address to Register:C
    Move = 0b00010000, // move the value in Register:A to a given memory address
    Add = 0b00100000, // add Register:B and Register:C and store the result in Register:A
    Jump = 0b01000000, // jump to the address given in the argument
    JumpIfOverflow = 0b01100000, // jump to the address given in the argument if overflow flag is 1
    Halt = 0b10000000, // halt the execution
}

impl Instruction {
    fn from_value(value: u8) -> Option<&'static str> {
        match value {
            0b00000001 => Some("LoadA"),
            0b00000010 => Some("LoadB"),
            0b00000011 => Some("LoadC"),
            0b00000100 => Some("StoreA"),
            0b00001000 => Some("StoreB"),
            0b00001100 => Some("StoreC"),
            0b00010000 => Some("Move"),
            0b00100000 => Some("Add"),
            0b01000000 => Some("Jump"),
            0b01100000 => Some("JumpIfOverflow"),
            0b10000000 => Some("Halt"),
            _ => None,
        }
    }
    
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
            i if i == Instruction::StoreA as u8 => {
                let address = machine.memory.read_word(pc + 1)?;
                machine.set_register(RegisterName::A, machine.memory.read_word(address)?);
                machine.set_register(RegisterName::PC, pc + 3);
            },
            i if i == Instruction::StoreB as u8 => {
                let address = machine.memory.read_word(pc + 1)?;
                machine.set_register(RegisterName::B, machine.memory.read_word(address)?);
                machine.set_register(RegisterName::PC, pc + 3);
            },
            i if i == Instruction::StoreC as u8 => {
                let address = machine.memory.read_word(pc + 1)?;
                machine.set_register(RegisterName::C, machine.memory.read_word(address)?);
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

                if let Some(result) = reg_b.checked_add(reg_c) {
                    machine.set_register(RegisterName::A, result);
                } else {
                    machine.set_flag(Flags::Overflow);
                }

                machine.set_register(RegisterName::PC, pc + 1);
            },
            i if i == Instruction::Jump as u8 => {
                let address = machine.memory.read_word(pc + 1)?;
                machine.set_register(RegisterName::PC, address);
            },
            i if i == Instruction::JumpIfOverflow as u8 => {
                if machine.get_flag(Flags::Overflow) {
                    let address = machine.memory.read_word(pc + 1)?;
                    machine.set_register(RegisterName::PC, address);
                } else {
                    machine.set_register(RegisterName::PC, pc + 3);
                }
            },
            i if i == Instruction::Halt as u8 => return Err("Halt".to_string()),
            _ => return Err("Unknown instruction".to_string()),
        }

        Ok(())
    }
}