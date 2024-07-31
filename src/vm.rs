use std::collections::HashMap;

#[allow(dead_code)]

#[derive(Debug)]
pub struct Machine {
    registers: HashMap<RegisterName, u16>,
    pub memory: [u8; 8096],
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum RegisterName {
    A, 
    B, 
    PC,
    FLAGS,
}

impl Machine {
    pub fn new() -> Self {
        let mut registers = HashMap::new();

        registers.insert(RegisterName::A, 0);
        registers.insert(RegisterName::B, 0);
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

    pub fn set_register(&mut self, register: RegisterName, value: u16) {
        if let Some(r) = self.registers.get_mut(&register) {
            *r = value;
        }
    }

    pub fn get_register(&self, register: RegisterName) -> u16 {
        match self.registers.get(&register) {
            Some(result) => return result.clone(),
            None => panic!("Something wrong with register {:?}", register),
        }
    }



}