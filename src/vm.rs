use std::collections::HashMap;

#[derive(Debug)]
pub struct Machine {
    registers: HashMap<RegisterName, u16>,
    pub memory: [u8; 8096],
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum RegisterName {
    A, 
    B,
    C,
    PC,
    FLAGS,
}

pub enum Flags {
    Carry = 0b0000_0001, // 0 - no carry, 1 - carry
    Sign = 0b0000_0010, // 0 - positive, 1 - negative
    Zero = 0b0000_0100, // 0 - non-zero, 1 - zero
    Overflow = 0b0000_1000, // 0 - no overflow, 1 - overflow
}

trait Flag {
    fn set_flag(&mut self, flag: Flags) -> u16;
    fn clear_flag(&mut self, flag: Flags);
    fn check_flag(self, flag: Flags) -> bool;
}

impl Flag for u16 {
    fn set_flag(&mut self, flag: Flags) -> u16 {
        *self |= flag as u16;
        *self
    }

    fn clear_flag(&mut self, flag: Flags) {
        *self &= !(flag as u16);
    }

    fn check_flag(self, flag: Flags) -> bool {
        self & (flag as u16) != 0
    }
}

impl Machine {
    pub fn new() -> Self {
        let mut registers = HashMap::new();

        registers.insert(RegisterName::A, 0);
        registers.insert(RegisterName::B, 0);
        registers.insert(RegisterName::C, 0);
        registers.insert(RegisterName::PC, 0);
        registers.insert(RegisterName::FLAGS, 0);

        Self { 
            registers, 
            memory: [0; 8096], 
        }
    }

    pub fn get_register_list(&self) -> &HashMap<RegisterName, u16> {
        &self.registers
    }

    pub fn get_register(&self, register: RegisterName) -> u16 {
        match self.registers.get(&register) {
            Some(result) => return result.clone(),
            None => panic!("Something wrong with register {:?}", register),
        }
    }
    pub fn set_register(&mut self, register: RegisterName, value: u16) {
        if let Some(r) = self.registers.get_mut(&register) {
            *r = value;
        }
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        match self.get_register(RegisterName::FLAGS).check_flag(flag) {
            true => return true,
            false => return false,
        }
    }

    pub fn set_flag(&mut self, flag: Flags) -> () {
        let mut flags_register_value = self.get_register(RegisterName::FLAGS);
        self.set_register(RegisterName::FLAGS, flags_register_value.set_flag(flag));

        ()
    }

}