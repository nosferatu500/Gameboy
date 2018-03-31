pub struct Flag {
    pub z: u8,
    pub n: u8,
    pub h: u8,
    pub c: u8,
}

impl Flag {
    fn new() -> Flag {
        Flag {
            z: 0,
            n: 0,
            h: 0,
            c: 0,
        }
    }
}

pub struct Register {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,

    pub flag: Flag,
}

impl Register {
    pub fn new() -> Register {
        Register {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,

            flag: Flag::new(),
        }
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f as u16
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xff00) >> 8) as u8;
        self.f = (value & 0xff) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xff00) >> 8) as u8;
        self.c = (value & 0xff) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xff00) >> 8) as u8;
        self.e = (value & 0xff) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xff00) >> 8) as u8;
        self.l = (value & 0xff) as u8;
    }
}
