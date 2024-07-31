pub trait Addressable {
    fn read(&self, address: u16) -> Result<u16, String> {
        Ok((self.read_byte(address)? as u16) << 8 | (self.read_byte(address + 1)? as u16))
    }

    fn write(&mut self, address: u16, value: u16) -> Result<(), String> {
        /* let value_msb = ((value & 0xff00) >> 8) as u8;
        let value_lsb = value as u8; */
        let be_value = value.to_be_bytes();
        let _ = self.write_byte(address, be_value[0]);
        let _ = self.write_byte(address + 1, be_value[1]);
        Ok(())
    }

    fn read_byte(&self, address: u16) -> Result<u8, String>;
    fn write_byte(&mut  self, address: u16, value: u8) -> Result<(), String>;
}

impl Addressable for [u8] {    
    fn read_byte(&self, address: u16) -> Result<u8, String> {
        if (address as usize) < self.len() {
            match self.get(address as usize) {
                Some(result) => return Ok(*result),
                None => return Err("Ejnyeno...".to_string()),
            }
        } else {
            Err("Ejnyeno...".to_string())
        }
    }
    
    fn write_byte(&mut self, address: u16, value: u8) -> Result<(), String> {
        if (address as usize) < self.len() {
            match self.get_mut(address as usize) {
                Some(byte) => {
                    *byte = value;
                    Ok(())
                },
                None => return Err("Ejnyeno...".to_string()),
            }
        } else {
            Err("Ejnyeno...".to_string())
        }
    }
}