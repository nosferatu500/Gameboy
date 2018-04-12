use mbc::MBC;

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_on: bool,
    ram_mode: bool,
    rom_bank: usize,
    ram_bank: usize,
}

impl MBC1 {
    pub fn new(data: Vec<u8>) -> ::StrResult<MBC1> {
        let size = match data[0x0147] {
            0x01 => 0,
            0x02 => ::mbc::ram_size(data[0x0149]),
            _ => 0,
        };

        Ok(MBC1 { 
            rom: data,
            ram: ::std::iter::repeat(0).take(size).collect(),
            ram_on: false,
            ram_mode: false,
            rom_bank: 1,
            ram_bank: 0,
        })
    }
}

impl MBC for MBC1 {
    fn readrom(&self, address: u16) -> u8 { 
        if address < 0x4000 {
            self.rom[address as usize] 
        } else {
            let addr = self.rom_bank * 0x4000 | ((address as usize) & 0x3FFF);
            self.rom[addr as usize]
        }
    }

    fn writerom(&mut self, address: u16, value: u8) { 
        match address {
            0x0000 ... 0x1FFF => {
                self.ram_on = (value & 0xFF) == 0x0A;
            }
            0x6000 ... 0x7FFF => { 
                self.ram_mode = (value & 0xFF) == 0x01;
            },
            _ => panic!("Unexpected write for MBC1"),
        }
    }
    
    fn readram(&self, address: u16) -> u8 { 
        if !self.ram_on {
            return 0;
        }
        if self.ram_mode { 
            self.ram[(self.ram_bank * 0x2000) | ((address & 0x1FFF) as usize)]
        } else { 
            self.ram[(address & 0x1FFF) as usize]
        }
     }

    fn writeram(&mut self, address: u16, value: u8) { 
        if !self.ram_on {
            return;
        }
        if self.ram_mode { 
            self.ram[(self.ram_bank * 0x2000) | ((address & 0x1FFF) as usize)] = value;
        } else { 
            self.ram[(address & 0x1FFF) as usize] = value;
        }
    }
}