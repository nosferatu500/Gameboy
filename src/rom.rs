use std::path::Path;
use std::fs::File;
use std::io::*;

pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Rom> {
        let mut file = try!(File::open(&path));

        let mut buf = Vec::new();

        try!(file.read_to_end(&mut buf));

        Ok(Rom { data: buf })
    }

    pub fn load(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn store(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}
