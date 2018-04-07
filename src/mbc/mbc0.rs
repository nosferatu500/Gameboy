use mbc::MBC;

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(data: Vec<u8>) -> ::StrResult<MBC0> {
        Ok(MBC0 { rom: data })
    }
}

impl MBC for MBC0 {
    fn readrom(&self, address: u16) -> u8 { 
        self.rom[address as usize] 
    }

    fn writerom(&mut self, _address: u16, _value: u8) { }
    
    fn readram(&self, _address: u16) -> u8 { 0 }

    fn writeram(&mut self, _address: u16, _value: u8) { }
}