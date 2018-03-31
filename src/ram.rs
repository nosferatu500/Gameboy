pub struct Ram {
  data: [u8; 8 * 1024],
}

impl Ram {
  pub fn new() -> Ram {
    Ram {
      data: [0; 8 * 1024],
    }
  }

  pub fn load(&self, address: u16) -> u8 {
      self.data[address as usize]
  }

  pub fn store(&mut self, address: u16, value: u8) {
      self.data[address as usize] = value;
  }
}
