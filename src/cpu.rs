use bus::Bus;
use register::Register;

pub struct Cpu {
    bus: Bus,

    pc: u16,
    sp: u16,

    register: Register,

    current_pc: u16,

    ime: bool,

    di: u32,
    ei: u32,

    log: bool,

    cartrige_type: u8,

    halted: bool,
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Cpu {
            bus,

            pc: 0x0100,
            sp: 0xFFFE,

            register: Register::new(),

            current_pc: 0x0,

            ime: true,

            di: 0,
            ei: 0,

            log: false,

            cartrige_type: 0x00,

            halted: false,
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.current_pc
    }

    pub fn update_ime(&mut self) {
        self.di = match self.di {
            2 => 1,
            1 => {
                self.ime = false;
                0
            }
            _ => 0,
        };
        self.ei = match self.ei {
            2 => 1,
            1 => {
                self.ime = true;
                0
            }
            _ => 0,
        };
    }

    pub fn test_draw(&mut self) {
      self.bus.gui.test_draw();
    }

    pub fn get_data_gui(&self, x: usize, y: usize) -> u32 {
      self.bus.gui.get_data(x, y)
    }

    pub fn enable_log(&mut self) {
      self.log = true;
    }

    fn update_register_f(&mut self) {
        self.register.f = (self.register.flag.z << 7) | (self.register.flag.h << 5)
            | (self.register.flag.n << 6) | (self.register.flag.c << 4);
    }

    fn push_stack(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.bus.store16(self.sp, value);
    }

    fn pop_stack(&mut self) -> u16 {
        let lhs = self.bus.load(self.sp) as u16;
        let rhs = self.bus.load(self.sp + 1) as u16;

        let instruction = ((lhs << 8) | rhs) as u16;

        self.sp = self.sp.wrapping_add(2);
        instruction
    }

    pub fn run_next_instruction(&mut self, callback: bool) {
        let instruction = self.bus.load(self.pc);

        self.current_pc = self.pc;

        //Only for debug purposes.
        if self.log {
          println!("");
          println!("| PC |: {:#06X}", self.pc);
          println!("");

          println!("| SP |: {:#06X}", self.sp);
          println!(" ");

          println!("| IN |: {:#04x}", instruction);
          println!(" ");
          let nn = (self.bus.load(self.current_pc + 2) as u16) << 8
              | self.bus.load(self.current_pc + 1) as u16;
          println!("| nn |: {:04X}", nn);
          println!(" ");
          let n = self.bus.load(self.current_pc + 1);
          println!("| n  |: {:02X}", n);
          println!(" ");

          println!("| Z  |: {:x}", self.register.flag.z);
          println!("| N  |: {:x}", self.register.flag.n);
          println!("| H  |: {:x}", self.register.flag.h);
          println!("| C  |: {:x}", self.register.flag.c);
          println!(" ");

          print!("| AF |: {:#06X}", self.register.af());
          print!("\t| LCDC FF40 |: {:#06X}", self.bus.load(0xFF40));
          
          println!(" ");

          print!("| BC |: {:#06X}", self.register.bc());
          print!("\t| STAT FF41 |: {:#06X}", self.bus.load(0xFF41));
          println!(" ");

          print!("| DE |: {:#06X}", self.register.de());
          print!("\t| LY FF44 |: {:#06X}", self.bus.load(0xFF44));
          println!(" ");

          println!("| HL |: {:#06X}", self.register.hl());
          println!(" ");

          println!("| SCY FF42 |: {:#06X}", self.bus.load(0xFF42));
          println!(" ");

          println!("| SCX FF43 |: {:#06X}", self.bus.load(0xFF43));
          println!(" ");

          println!("| LYC FF45 |: {:#06X}", self.bus.load(0xFF45));
          println!(" ");

          //println!("| DMA FF46 |: {:#06X}", self.bus.load(0xFF46));
          println!(" ");

          println!("| BGP FF47 |: {:#06X}", self.bus.load(0xFF47));
          println!(" ");

          println!("| OBP0 FF48 |: {:#06X}", self.bus.load(0xFF48));
          println!(" ");

          println!("| OBP1 FF49 |: {:#06X}", self.bus.load(0xFF49));
          println!(" ");

          println!("| WY FF4A |: {:#06X}", self.bus.load(0xFF4A));
          println!(" ");

          println!("| WX FF4B |: {:#06X}", self.bus.load(0xFF4B));
          println!(" ");
          println!(" ");

          println!("| TYPE 0147 |: {:#06X}", self.bus.load(0x0147));
          println!(" ");

          println!("| ROM 0148 |: {:#06X}", self.bus.load(0x0148));
          println!(" ");

          println!("| RAM 0149 |: {:#06X}", self.bus.load(0x0149));
          println!(" ");
          println!("***********************************");
        }

        self.pc = self.pc.wrapping_add(1);

        if callback {
            self.decode_callback(instruction);
        } else {
            self.decode(instruction);        
        }

    }

    pub fn power_up(&mut self) {
        self.register.set_af(0x01B0);

        self.register.f = 0xb0;

        self.sp = 0xFFFE;

        self.register.set_bc(0x0013);
        self.register.set_de(0x00D8);
        self.register.set_hl(0x014D);

        self.bus.store(0xFF05, 0x00);
        self.bus.store(0xFF06, 0x00);
        self.bus.store(0xFF07, 0x00);
        self.bus.store(0xFF10, 0x80);
        self.bus.store(0xFF11, 0xBF);
        self.bus.store(0xFF12, 0xF3);
        self.bus.store(0xFF14, 0xBF);
        self.bus.store(0xFF16, 0x3F);
        self.bus.store(0xFF17, 0x00);
        self.bus.store(0xFF19, 0xBF);
        self.bus.store(0xFF1A, 0x7F);
        self.bus.store(0xFF1B, 0xFF);
        self.bus.store(0xFF1C, 0x9F);
        self.bus.store(0xFF1E, 0xBF);
        self.bus.store(0xFF20, 0xFF);
        self.bus.store(0xFF21, 0x00);
        self.bus.store(0xFF22, 0x00);
        self.bus.store(0xFF23, 0xBF);
        self.bus.store(0xFF24, 0x77);
        self.bus.store(0xFF25, 0xF3);
        self.bus.store(0xFF26, 0xF1);
        self.bus.store(0xFF40, 0x91);
        self.bus.store(0xFF42, 0x00);
        self.bus.store(0xFF43, 0x00);
        self.bus.store(0xFF45, 0x00);
        self.bus.store(0xFF47, 0xFC);
        self.bus.store(0xFF48, 0xFF);
        self.bus.store(0xFF49, 0xFF);
        self.bus.store(0xFF4A, 0x00);
        self.bus.store(0xFF4B, 0x00);
        self.bus.store(0xFFFF, 0x00);
    }

    fn decode_callback(&mut self, instruction: u8) {
        let nn = (self.bus.load(self.current_pc + 2) as u16) << 8
            | self.bus.load(self.current_pc + 1) as u16;
        let n = self.bus.load(self.current_pc + 1);

        match instruction {
            0x30 => {
                let value = self.register.b;
                self.register.b = (value >> 4) | (value << 4);

                self.register.flag.z = (value == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x31 => {
                let value = self.register.c;
                self.register.c = (value >> 4) | (value << 4);

                self.register.flag.z = (value == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x32 => {
                let value = self.register.d;
                self.register.d = (value >> 4) | (value << 4);

                self.register.flag.z = (value == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x33 => {
                let value = self.register.e;
                self.register.e = (value >> 4) | (value << 4);

                self.register.flag.z = (value == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x34 => {
                let value = self.register.h;
                self.register.h = (value >> 4) | (value << 4);

                self.register.flag.z = (value == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();
                
                self.bus.add_to_clock(8);
            }
            0x35 => {
                let value = self.register.l;
                self.register.l = (value >> 4) | (value << 4);

                self.register.flag.z = (value == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.bus.add_to_clock(8);
            }
            0x36 => {
                let value = self.bus.load(self.register.hl());
                self.bus.store(self.register.hl(), (value >> 4) | (value << 4));

                self.register.flag.z = (value == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(16);
            }
            0x37 => {
                let value = self.register.a;
                self.register.a = (value >> 4) | (value << 4);

                self.register.flag.z = (value == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x80 => {
                self.register.b = self.register.b & !(1 << 0);
                self.bus.add_to_clock(8);
            }
            0x81 => {
                self.register.c = self.register.c & !(1 << 0);
                self.bus.add_to_clock(8);
            }
            0x82 => {
                self.register.d = self.register.d & !(1 << 0);
                self.bus.add_to_clock(8);
            }
            0x83 => {
                self.register.e = self.register.e & !(1 << 0);
                self.bus.add_to_clock(8);
            }
            0x84 => {
                self.register.h = self.register.h & !(1 << 0);
                self.bus.add_to_clock(8);
            }
            0x85 => {
                self.register.l = self.register.l & !(1 << 0);
                self.bus.add_to_clock(8);
            }
            0x86 => {
                let value = self.bus.load(self.register.hl());
                self.bus.store(self.register.hl(), value & !(1 << 0));

                self.bus.add_to_clock(16);
            }
            0x87 => {
                self.register.a = self.register.a & !(1 << 0);
                self.bus.add_to_clock(8);
            }
            _ => panic!("Unknown callback instruction {:#04x}", instruction),
        }
    }

    fn decode(&mut self, instruction: u8) {
        let nn = (self.bus.load(self.current_pc + 2) as u16) << 8
            | self.bus.load(self.current_pc + 1) as u16;
        let n = self.bus.load(self.current_pc + 1);

        match instruction {
            0x00 => self.bus.add_to_clock(4),
            0x01 => {
                self.register.set_bc(nn);
                self.bus.add_to_clock(12);
            }
            0x02 => {
                self.bus.store(self.register.bc(), self.register.a);
                self.bus.add_to_clock(8);
            }
            0x03 => {
                let value = self.bus.load(self.register.bc()).wrapping_add(1);
                self.bus.store(self.register.bc(), value);

                self.bus.add_to_clock(8);
            }
            0x04 => {
                let old_value = self.register.b;
                self.register.b = self.register.b.wrapping_add(1);

                self.register.flag.z = (self.register.b == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x0F) + 1 > 0x0F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x05 => {
                let old_value = self.register.b;
                self.register.b = old_value.wrapping_sub(1);

                self.register.flag.z = (self.register.b == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) == 0) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x06 => {
                self.register.b = n;
                self.bus.add_to_clock(8);
                self.pc = self.pc.wrapping_add(1);
            }
            0x07 => {
                //TODO: Probably incorrect rotate_left value. Maybe we need use << operator.
                let old_bit = (self.register.a >> 7) & 0b1;
                self.register.a = self.register.a.rotate_left(1);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = old_bit;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x08 => {
                self.bus.store16(nn, self.sp);
                self.bus.add_to_clock(20);
            }
            0x09 => {
                let value = self.bus
                    .load(self.register.bc())
                    .wrapping_add(self.bus.load(self.register.hl()));

                self.bus.store(self.register.hl(), value);

                self.register.flag.n = 0;
                self.register.flag.h = (((value as u16) & 0x7FF) + 1 > 0x7FF) as u8;
                self.register.flag.c = (((value as u16) & 0x7FFF) + 1 > 0x7FFF) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x0A => {
                let value = self.bus.load(self.register.bc());

                self.register.a = value;

                self.bus.add_to_clock(8);

                self.pc = self.pc.wrapping_add(2);
            }
            0x0B => {
                let mut value = self.bus.load(self.register.bc());
                value = value.wrapping_sub(1);

                self.bus.store(self.register.bc(), value);

                self.bus.add_to_clock(8);
            }
            0x0C => {
                let old_value = self.register.c;
                self.register.c = self.register.c.wrapping_add(1);

                self.register.flag.z = (self.register.c == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x0F) + 1 > 0x0F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x0D => {
                let old_value = self.register.c;
                self.register.c = old_value.wrapping_sub(1);

                self.register.flag.z = (self.register.c == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = (old_value & 0x0F == 0) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x0E => {
                self.register.c = n;
                self.bus.add_to_clock(8);
                self.pc = self.pc.wrapping_add(1);
            }
            0x0F => {
                //TODO: Probably incorrect rotate_right value. Maybe we need use >> operator.
                let old_bit = self.register.a & 0b1;
                self.register.a = self.register.a.rotate_right(1);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = old_bit;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x11 => {
                self.register.set_de(nn);
                self.bus.add_to_clock(12);
            }
            0x12 => {
                self.bus.store(self.register.de(), self.register.a);
                self.bus.add_to_clock(8);
            }
            0x13 => {
                let value = self.bus.load(self.register.de()).wrapping_add(1);
                self.bus.store(self.register.de(), value);

                self.bus.add_to_clock(8);
            }
            0x14 => {
                let old_value = self.register.d;
                self.register.d = self.register.d.wrapping_add(1);

                self.register.flag.z = (self.register.d == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x0F) + 1 > 0x0F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x15 => {
                let old_value = self.register.d;
                self.register.d = self.register.d.wrapping_sub(1);

                self.register.flag.z = (self.register.d == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = (old_value & 0x0F == 0) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x16 => {
                self.register.d = n;
                self.bus.add_to_clock(8);
            }
            0x19 => {
                let value = self.bus
                    .load(self.register.de())
                    .wrapping_add(self.bus.load(self.register.hl()));

                self.bus.store(self.register.hl(), value);

                self.register.flag.n = 0;
                self.register.flag.h = ((value & 0x7FF) + 1 > 0x7FF) as u8;
                self.register.flag.c = ((value & 0x7FFF) + 1 > 0x7FFF) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x1A => {
                let value = self.bus.load(self.register.de());

                self.register.a = value;

                self.bus.add_to_clock(8);

                self.pc = self.pc.wrapping_add(2);
            }
            0x1B => {
                let mut value = self.bus.load(self.register.de());
                value = value.wrapping_sub(1);

                self.bus.store(self.register.de(), value);

                self.bus.add_to_clock(8);
            }
            0x1C => {
                let old_value = self.register.e;
                self.register.e = self.register.e.wrapping_add(1);

                self.register.flag.z = (self.register.e == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x0F) + 1 > 0x0F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x1D => {
                let old_value = self.register.e;
                self.register.e = self.register.e.wrapping_sub(1);

                self.register.flag.z = (self.register.e == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = (old_value & 0x0F == 0) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x1E => {
                self.register.e = n;
                self.bus.add_to_clock(8);
            }
            0x1F => {
                let old_bit = self.register.a & 0b1;
                self.register.a = self.register.a.rotate_right(self.register.c as u32);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = old_bit;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x20 => {
                if self.register.flag.z == 0 {
                    self.pc = self.pc.wrapping_add((n as i8) as u16);
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(8);
            }
            0x21 => {
                self.register.set_hl(nn);
                self.bus.add_to_clock(12);
                self.pc = self.pc.wrapping_add(2);
            }
            0x23 => {
                let value = self.bus.load(self.register.hl()).wrapping_add(1);
                self.bus.store(self.register.hl(), value);

                self.bus.add_to_clock(8);
            }
            0x24 => {
                let old_value = self.register.h;
                self.register.h = self.register.h.wrapping_add(1);

                self.register.flag.z = (self.register.h == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x0F) + 1 > 0x0F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x25 => {
                let old_value = self.register.h;
                self.register.h = self.register.h.wrapping_sub(1);

                self.register.flag.z = (self.register.h == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = (old_value & 0x0F == 0) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x26 => {
                self.register.h = n;
                self.bus.add_to_clock(8);
            }
            0x28 => {
                if self.register.flag.z == 1 {
                    self.pc = self.pc.wrapping_add(n as u16);
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(8);
            }
            0x29 => {
                let value = self.bus
                    .load(self.register.hl())
                    .wrapping_add(self.bus.load(self.register.hl()));

                self.bus.store(self.register.hl(), value);

                self.register.flag.n = 0;
                self.register.flag.h = ((value & 0x7FF) + 1 > 0x7FF) as u8;
                self.register.flag.c = ((value & 0x7FFF) + 1 > 0x7FFF) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x2B => {
                let mut value = self.bus.load(self.register.hl());
                value = value.wrapping_sub(1);

                self.bus.store(self.register.hl(), value);

                self.bus.add_to_clock(8);
            }
            0x2C => {
                let old_value = self.register.l;
                self.register.l = self.register.l.wrapping_add(1);

                self.register.flag.z = (self.register.l == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x0F) + 1 > 0x0F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x2D => {
                let old_value = self.register.d;
                self.register.d = self.register.d.wrapping_sub(1);

                self.register.flag.z = (self.register.d == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = (old_value & 0x0F == 0) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x2E => {
                self.register.l = n;
                self.bus.add_to_clock(8);
            }
            0x30 => {
                if self.register.flag.c == 0 {
                    self.pc = self.pc.wrapping_add(n as u16);
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(8);
            }
            0x31 => {
                self.sp = nn;
                self.bus.add_to_clock(12);
                self.pc = self.pc.wrapping_add(2);
            }
            0x32 => {
                let hl = self.register.hl();
                self.bus.store(hl, self.register.a);
                self.bus.add_to_clock(8);

                self.register.set_hl(hl.wrapping_sub(1));
            }
            0x33 => {
                let value = self.sp.wrapping_add(1);
                self.bus.store16(self.sp, value);

                self.bus.add_to_clock(8);
            }
            0x34 => {
                let mut value = self.bus.load(self.register.hl());
                let old_value = value;
                value = value.wrapping_add(1);

                self.register.flag.z = (value == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x0F) + 1 > 0x0F) as u8;

                self.update_register_f();

                self.bus.store(self.register.hl(), value);

                self.bus.add_to_clock(12);
            }
            0x35 => {
                let mut value = self.bus.load(self.register.hl());
                let old_value = value;
                value = value.wrapping_sub(1);

                self.register.flag.z = (value == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = (old_value & 0x0F == 0) as u8;

                self.update_register_f();

                self.bus.store(self.register.hl(), value);

                self.bus.add_to_clock(12);
            }
            0x36 => {
                self.bus.store(self.register.hl(), n);
                self.bus.add_to_clock(12);
            }
            0x38 => {
                if self.register.flag.c == 1 {
                    self.pc = self.pc.wrapping_add(n as u16);
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(8);
            }
            0x39 => {
                let value = self.sp
                    .wrapping_add(self.bus.load(self.register.hl()) as u16);

                self.bus.store16(self.register.hl(), value);

                self.register.flag.n = 0;
                self.register.flag.h = ((value & 0x7FF) + 1 > 0x7FF) as u8;
                self.register.flag.c = ((value & 0x7FFF) + 1 > 0x7FFF) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x3A => {
                let value = self.bus.load(self.register.hl());
                self.register.a = value;

                self.bus.store(self.register.hl(), value.wrapping_sub(1));

                self.bus.add_to_clock(8);
            }
            0x3B => {
                self.sp = self.sp.wrapping_sub(1);
                self.bus.add_to_clock(8);
            }
            0x3C => {
                let old_value = self.register.a;
                self.register.a = self.register.a.wrapping_add(1);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x0F) + 1 > 0x0F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x3D => {
                let old_value = self.register.a;
                self.register.a = self.register.a.wrapping_sub(1);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = (old_value & 0x0F == 0) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x3E => {
                self.register.a = n;

                self.bus.add_to_clock(8);

                self.pc = self.pc.wrapping_add(2);
            }
            0x40 => {
                self.bus.add_to_clock(4);
            }
            0x41 => {
                self.register.b = self.register.c;
                self.bus.add_to_clock(4);
            }
            0x42 => {
                self.register.b = self.register.d;
                self.bus.add_to_clock(4);
            }
            0x43 => {
                self.register.b = self.register.e;
                self.bus.add_to_clock(4);
            }
            0x44 => {
                self.register.b = self.register.h;
                self.bus.add_to_clock(4);
            }
            0x45 => {
                self.register.b = self.register.l;
                self.bus.add_to_clock(4);
            }
            0x46 => {
                self.register.b = self.bus.load(self.register.hl());
                self.bus.add_to_clock(8);
            }
            0x47 => {
                self.register.b = self.register.a;
                self.bus.add_to_clock(4);
            }
            0x48 => {
                self.register.c = self.register.b;
                self.bus.add_to_clock(4);
            }
            0x49 => {
                self.bus.add_to_clock(4);
            }
            0x4A => {
                self.register.c = self.register.d;
                self.bus.add_to_clock(4);
            }
            0x4B => {
                self.register.c = self.register.e;
                self.bus.add_to_clock(4);
            }
            0x4C => {
                self.register.c = self.register.h;
                self.bus.add_to_clock(4);
            }
            0x4D => {
                self.register.c = self.register.l;
                self.bus.add_to_clock(4);
            }
            0x4E => {
                self.register.c = self.bus.load(self.register.hl());
                self.bus.add_to_clock(8);
            }
            0x4F => {
                self.register.c = self.register.a;
                self.bus.add_to_clock(4);
            }
            0x50 => {
                self.register.d = self.register.b;
                self.bus.add_to_clock(4);
            }
            0x51 => {
                self.register.d = self.register.c;
                self.bus.add_to_clock(4);
            }
            0x52 => {
                self.bus.add_to_clock(4);
            }
            0x53 => {
                self.register.d = self.register.e;
                self.bus.add_to_clock(4);
            }
            0x54 => {
                self.register.d = self.register.h;
                self.bus.add_to_clock(4);
            }
            0x55 => {
                self.register.d = self.register.l;
                self.bus.add_to_clock(4);
            }
            0x56 => {
                self.register.d = self.bus.load(self.register.hl());
                self.bus.add_to_clock(8);
            }
            0x57 => {
                self.register.d = self.register.a;
                self.bus.add_to_clock(4);
            }
            0x58 => {
                self.register.e = self.register.b;
                self.bus.add_to_clock(4);
            }
            0x59 => {
                self.register.e = self.register.c;
                self.bus.add_to_clock(4);
            }
            0x5A => {
                self.register.e = self.register.d;
                self.bus.add_to_clock(4);
            }
            0x5B => {
                self.bus.add_to_clock(4);
            }
            0x5C => {
                self.register.e = self.register.h;
                self.bus.add_to_clock(4);
            }
            0x5D => {
                self.register.e = self.register.l;
                self.bus.add_to_clock(4);
            }
            0x5E => {
                self.register.e = self.bus.load(self.register.hl());
                self.bus.add_to_clock(8);
            }
            0x5F => {
                self.register.e = self.register.a;
                self.bus.add_to_clock(4);
            }
            0x60 => {
                self.register.h = self.register.b;
                self.bus.add_to_clock(4);
            }
            0x61 => {
                self.register.h = self.register.c;
                self.bus.add_to_clock(4);
            }
            0x62 => {
                self.register.h = self.register.d;
                self.bus.add_to_clock(4);
            }
            0x63 => {
                self.register.h = self.register.e;
                self.bus.add_to_clock(4);
            }
            0x64 => {
                self.bus.add_to_clock(4);
            }
            0x65 => {
                self.register.h = self.register.l;
                self.bus.add_to_clock(4);
            }
            0x66 => {
                self.register.h = self.bus.load(self.register.hl());
                self.bus.add_to_clock(8);
            }
            0x67 => {
                self.register.h = self.register.a;
                self.bus.add_to_clock(4);
            }
            0x68 => {
                self.register.l = self.register.b;
                self.bus.add_to_clock(4);
            }
            0x69 => {
                self.register.l = self.register.c;
                self.bus.add_to_clock(4);
            }
            0x6A => {
                self.register.l = self.register.d;
                self.bus.add_to_clock(4);
            }
            0x6B => {
                self.register.l = self.register.e;
                self.bus.add_to_clock(4);
            }
            0x6C => {
                self.register.l = self.register.h;
                self.bus.add_to_clock(4);
            }
            0x6D => {
                self.bus.add_to_clock(4);
            }
            0x6E => {
                self.register.l = self.bus.load(self.register.hl());
                self.bus.add_to_clock(8);
            }
            0x6F => {
                self.register.l = self.register.a;
                self.bus.add_to_clock(4);
            }
            0x70 => {
                self.bus.store(self.register.hl(), self.register.b);
                self.bus.add_to_clock(8);
            }
            0x71 => {
                self.bus.store(self.register.hl(), self.register.c);
                self.bus.add_to_clock(8);
            }
            0x72 => {
                self.bus.store(self.register.hl(), self.register.d);
                self.bus.add_to_clock(8);
            }
            0x73 => {
                self.bus.store(self.register.hl(), self.register.e);
                self.bus.add_to_clock(8);
            }
            0x74 => {
                self.bus.store(self.register.hl(), self.register.h);
                self.bus.add_to_clock(8);
            }
            0x75 => {
                self.bus.store(self.register.hl(), self.register.l);
                self.bus.add_to_clock(8);
            }
            0x76 => {
                self.halted = true;
                self.bus.add_to_clock(4);
            }
            0x77 => {
                self.bus.store(self.register.hl(), self.register.a);
                self.bus.add_to_clock(8);
            }
            0x78 => {
                self.register.a = self.register.b;
                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(2);
            }
            0x79 => {
                self.register.a = self.register.c;
                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(2);
            }
            0x7A => {
                self.register.a = self.register.d;
                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(2);
            }
            0x7B => {
                self.register.a = self.register.e;
                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(2);
            }
            0x7C => {
                self.register.a = self.register.h;
                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(2);
            }
            0x7D => {
                self.register.a = self.register.l;
                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(2);
            }
            0x7E => {
                self.register.a = self.bus.load(self.register.hl());
                self.bus.add_to_clock(8);
                self.pc = self.pc.wrapping_add(2);
            }
            0x7F => {
                self.bus.add_to_clock(4);
            }
            0x80 => {
                let old_value = self.register.a;
                self.register.a = self.register.a.wrapping_add(self.register.b);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x81 => {
                let old_value = self.register.a;
                self.register.a = self.register.a.wrapping_add(self.register.c);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x82 => {
                let old_value = self.register.a;
                self.register.a = self.register.a.wrapping_add(self.register.d);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x83 => {
                let old_value = self.register.a;
                self.register.a = self.register.a.wrapping_add(self.register.e);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x84 => {
                let old_value = self.register.a;
                self.register.a = self.register.a.wrapping_add(self.register.h);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x85 => {
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_add(self.register.l);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x86 => {
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_add(self.bus.load(self.register.hl()));

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x87 => {
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_add(self.register.a);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x88 => {
                let old_value = self.register.a;
                self.register.a =
                    old_value.wrapping_add(self.register.b.wrapping_add(self.register.flag.c));

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x89 => {
                let old_value = self.register.a;
                self.register.a =
                    old_value.wrapping_add(self.register.c.wrapping_add(self.register.flag.c));

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x8A => {
                let old_value = self.register.a;
                self.register.a =
                    old_value.wrapping_add(self.register.d.wrapping_add(self.register.flag.c));

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x8B => {
                let old_value = self.register.a;
                self.register.a =
                    old_value.wrapping_add(self.register.e.wrapping_add(self.register.flag.c));

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x8C => {
                let old_value = self.register.a;
                self.register.a =
                    old_value.wrapping_add(self.register.h.wrapping_add(self.register.flag.c));

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x8D => {
                let old_value = self.register.a;
                self.register.a =
                    old_value.wrapping_add(self.register.l.wrapping_add(self.register.flag.c));

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x8E => {
                let old_value = self.register.a;
                self.register.a = old_value
                    .wrapping_add(self.bus.load(self.register.hl()))
                    .wrapping_add(self.register.flag.c);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x8F => {
                let old_value = self.register.a;
                self.register.a =
                    old_value.wrapping_add(old_value.wrapping_add(self.register.flag.c));

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x90 => {
                let value = self.register.b;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x91 => {
                let value = self.register.c;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x92 => {
                let value = self.register.d;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x93 => {
                let value = self.register.e;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x94 => {
                let value = self.register.h;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x95 => {
                let value = self.register.l;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x96 => {
                let value = self.bus.load(self.register.hl());
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0x97 => {
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(old_value);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (old_value & 0x0F)) as u8;
                self.register.flag.c = (old_value < old_value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x98 => {
                let value = self.register.b;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value).wrapping_add(self.register.c);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x99 => {
                let value = self.register.c;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value).wrapping_add(self.register.c);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x9A => {
                let value = self.register.d;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value).wrapping_add(self.register.c);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x9B => {
                let value = self.register.e;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value).wrapping_add(self.register.c);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x9C => {
                let value = self.register.h;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value).wrapping_add(self.register.c);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x9D => {
                let value = self.register.l;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value).wrapping_add(self.register.c);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x9E => {
                let value = self.bus.load(self.register.hl());
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value).wrapping_add(self.register.c);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0x9F => {
                let value = self.register.a;
                
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(value).wrapping_add(self.register.c);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xA0 => {
                self.register.a = self.register.a & self.register.b;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 1;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xA1 => {
                self.register.a = self.register.a & self.register.c;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 1;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xA2 => {
                self.register.a = self.register.a & self.register.d;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 1;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xA3 => {
                self.register.a = self.register.a & self.register.e;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 1;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xA4 => {
                self.register.a = self.register.a & self.register.h;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 1;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xA5 => {
                self.register.a = self.register.a & self.register.l;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 1;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xA6 => {
                let value = self.bus.load(self.register.hl());
                self.register.a = self.register.a & value;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 1;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0xA7 => {
                self.register.a = self.register.a & self.register.a;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 1;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xA8 => {
                self.register.a = self.register.a ^ self.register.b;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xA9 => {
                self.register.a = self.register.a ^ self.register.c;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xAA => {
                self.register.a = self.register.a ^ self.register.d;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xAB => {
                self.register.a = self.register.a ^ self.register.e;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xAC => {
                self.register.a = self.register.a ^ self.register.h;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xAD => {
                self.register.a = self.register.a ^ self.register.l;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xAE => {
                self.register.a = self.register.a ^ self.bus.load(self.register.hl());

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0xAF => {
                self.register.a = self.register.a ^ self.register.a;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xB0 => {
                self.register.a = self.register.a | self.register.b;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xB1 => {
                self.register.a = self.register.a | self.register.c;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xB2 => {
                self.register.a = self.register.a | self.register.d;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xB3 => {
                self.register.a = self.register.a | self.register.e;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xB4 => {
                self.register.a = self.register.a | self.register.h;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xB5 => {
                self.register.a = self.register.a | self.register.l;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xB6 => {
                let value = self.bus.load(self.register.hl());
                self.register.a = self.register.a | value;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0xB7 => {
                self.register.a = self.register.a | self.register.a;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(4);
            }
            0xB8 => {
                let value = self.register.b;

                let a = self.register.a.wrapping_sub(value);

                let old_value = self.register.a;

                self.register.flag.z = (a == value) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(1);
            }
            0xB9 => {
                let value = self.register.c;

                let old_value = self.register.a;

                let a = self.register.a.wrapping_sub(value);

                self.register.flag.z = (a == value) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(1);
            }
            0xBA => {
                let value = self.register.d;

                let old_value = self.register.a;

                let a = self.register.a.wrapping_sub(value);

                self.register.flag.z = (a == value) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(1);
            }
            0xBB => {
                let value = self.register.e;

                let old_value = self.register.a;

                let a = self.register.a.wrapping_sub(value);

                self.register.flag.z = (a == value) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(1);
            }
            0xBC => {
                let value = self.register.h;

                let old_value = self.register.a;
                let a = self.register.a.wrapping_sub(value);

                self.register.flag.z = (a == value) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(1);
            }
            0xBD => {
                let value = self.register.l;

                let old_value = self.register.a;

                let a = self.register.a.wrapping_sub(value);

                self.register.flag.z = (a == value) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(1);
            }
            0xBE => {
                let value = self.bus.load(self.register.hl());

                let old_value = self.register.a;
                let a = self.register.a.wrapping_sub(value);

                self.register.flag.z = (a == value) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
                self.pc = self.pc.wrapping_add(1);
            }
            0xBF => {
                let value = self.register.a;

                let old_value = self.register.a;

                let a = self.register.a.wrapping_sub(value);

                self.register.flag.z = (a == value) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(4);
                self.pc = self.pc.wrapping_add(1);
            }
            0xC0 => {
                if self.register.flag.z == 0 {
                    self.pop_stack();
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(8);
            }
            0xC1 => {
                let value = self.pop_stack();
                self.bus.store16(self.register.bc(), value);
                self.bus.add_to_clock(12);
            }
            0xC2 => {
                if self.register.flag.c == 0 {
                    self.pc = nn;
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(12);
            }
            0xC3 => {
                self.pc = nn;
                self.bus.add_to_clock(12);
            }
            0xC4 => {
                if self.register.flag.z == 0 {
                    let addr = self.current_pc + 2;
                    self.push_stack(addr);
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(12);
            }
            0xC6 => {
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_add(n);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0xC7 => {
                let value = self.current_pc;
                self.push_stack(value);

                self.pc = 0x00;

                self.bus.add_to_clock(32);
            }
            0xC8 => {
                if self.register.flag.z == 1 {
                    self.pop_stack();
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(8);
            }
            0xC9 => {
                self.pc = self.pop_stack();

                self.bus.add_to_clock(8);
            }
            0xCA => {
                if self.register.flag.z == 1 {
                    self.pc = nn;
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(12);
            }
            0xCB => {
                self.run_next_instruction(true);
            }
            0xCC => {
                if self.register.flag.z == 1 {
                    let addr = self.current_pc + 2;
                    self.push_stack(addr);
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(12);
            }
            0xCD => {
                let addr = self.current_pc + 2;
                self.push_stack(addr);
                self.pc = nn;
                self.bus.add_to_clock(12);
            }
            0xCE => {
                let old_value = self.register.a;
                self.register.a = old_value
                    .wrapping_add(n)
                    .wrapping_add(self.register.flag.c);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = ((old_value & 0x7) + 1 > 0x7) as u8;
                self.register.flag.c = ((old_value & 0x7F) + 1 > 0x7F) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0xCF => {
                let value = self.current_pc;
                self.push_stack(value);

                self.pc = 0x08;

                self.bus.add_to_clock(32);
            }
            0xD0 => {
                if self.register.flag.c == 0 {
                    self.pop_stack();
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(8);
            }
            0xD1 => {
                let value = self.pop_stack();
                self.bus.store16(self.register.de(), value);
                self.bus.add_to_clock(12);
            }
            0xD2 => {
                if self.register.flag.c == 0 {
                    self.pc = nn;
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(12);
            }
            0xD4 => {
                if self.register.flag.c == 0 {
                    let addr = self.current_pc + 2;
                    self.push_stack(addr);
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(12);
            }
            0xD6 => {
                let old_value = self.register.a;
                self.register.a = old_value.wrapping_sub(n);

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (n & 0x0F)) as u8;
                self.register.flag.c = (old_value < n) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0xD7 => {
                let value = self.current_pc;
                self.push_stack(value);

                self.pc = 0x10;

                self.bus.add_to_clock(32);
            }
            0xD8 => {
                if self.register.flag.c == 1 {
                    self.pop_stack();
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(8);
            }
            0xDA => {
                if self.register.flag.c == 1 {
                    self.pc = nn;
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(12);
            }
            0xDC => {
                if self.register.flag.c == 1 {
                    let addr = self.current_pc + 2;
                    self.push_stack(addr);
                }
                self.pc = self.pc.wrapping_add(1);
                self.bus.add_to_clock(12);
            }
            0xDF => {
                let value = self.current_pc;
                self.push_stack(value);

                self.pc = 0x18;

                self.bus.add_to_clock(32);
            }
            0xE0 => {
                self.bus.store(0xFF00 | n as u16, self.register.a);
                self.bus.add_to_clock(12);

                self.pc = self.pc.wrapping_add(1);
            }
            0xE1 => {
                let value = self.pop_stack();
                self.bus.store16(self.register.hl(), value);
                self.bus.add_to_clock(12);
            }
            0xE2 => {
                self.bus
                    .store(0xFF00 | self.register.c as u16, self.register.a);
                self.bus.add_to_clock(8);
            }
            0xE6 => {
                self.register.a = self.register.a & n;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 1;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0xE7 => {
                let value = self.current_pc;
                self.push_stack(value);

                self.pc = 0x20;

                self.bus.add_to_clock(32);
            }
            0xE8 => {
                self.sp = self.sp.wrapping_add(n as u16);

                self.register.flag.z = 0;
                self.register.flag.n = 0;
                self.register.flag.h = ((self.sp & 0x000F) > 0x000F) as u8;
                self.register.flag.c = ((self.sp & 0x00FF) > 0x00FF) as u8;

                self.update_register_f();

                self.bus.add_to_clock(16);
            }
            0xEA => {
                self.bus.store(nn, self.register.a);
                self.bus.add_to_clock(16);
            }
            0xEE => {
                self.register.a = self.register.a ^ n;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0xEF => {
                let value = self.current_pc;
                self.push_stack(value);

                self.pc = 0x28;

                self.bus.add_to_clock(32);
            }
            0xF0 => {
                self.register.a = self.bus.load(0xFF00 | n as u16);
                self.bus.add_to_clock(12);

                self.pc = self.pc.wrapping_add(1);
            }
            0xF1 => {
                let value = self.pop_stack();
                self.bus.store16(self.register.af(), value);
                self.bus.add_to_clock(12);
            }
            0xF2 => {
                let value = self.bus.load(0xFF00 | self.register.c as u16);
                self.register.a = value;
                self.bus.add_to_clock(8);
            }
            0xF3 => {
                self.di = 2; //Execute after after instruction (jump through 2);
                self.bus.add_to_clock(4);
            }
            0xF6 => {
                self.register.a = self.register.a | n;

                self.register.flag.z = (self.register.a == 0) as u8;
                self.register.flag.n = 0;
                self.register.flag.h = 0;
                self.register.flag.c = 0;

                self.update_register_f();

                self.bus.add_to_clock(8);
            }
            0xF7 => {
                let value = self.current_pc;
                self.push_stack(value);

                self.pc = 0x30;

                self.bus.add_to_clock(32);
            }
            0xF8 => {
                self.bus.store16(self.register.hl(), self.sp + n as u16);
                self.bus.add_to_clock(12);

                self.pc = self.pc.wrapping_add(1);
            }
            0xFA => {
                let value = self.bus.load(nn);

                self.register.a = value;

                self.bus.add_to_clock(16);
                self.pc = self.pc.wrapping_add(2);
            }
            0xFE => {
                let value = n;

                let old_value = self.register.a;

                let a = self.register.a.wrapping_sub(value);

                self.register.flag.z = (a == value) as u8;
                self.register.flag.n = 1;
                self.register.flag.h = ((old_value & 0x0F) < (value & 0x0F)) as u8;
                self.register.flag.c = (old_value < value) as u8;

                self.update_register_f();

                self.bus.add_to_clock(8);
                self.pc = self.pc.wrapping_add(1);
            }
            0xFF => {
                let value = self.current_pc;
                self.push_stack(value);

                self.pc = 0x38;

                self.bus.add_to_clock(32);
            }
            _ => {
                println!("self.pc: {:#04x}", self.current_pc);
                panic!("Unknown instruction {:#04x}", instruction);
            }
        }
    }
}
