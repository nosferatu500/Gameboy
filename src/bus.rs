use clock::Clock;
use sound::Sound;
use gui::*;
use joypad::Joypad;
use serial::Serial;

mod map {
    pub struct Range(u16, u16);

    impl Range {
        pub fn contains(self, addr: u16) -> Option<u16> {
            let Range(start, end) = self;

            if addr >= start && addr < start + (end - start + 0x1) {
                Some(addr - start)
            } else {
                None
            }
        }
    }

    pub const ROM: Range = Range(0x0000, 0x3FFF);

    pub const SWITCHABLE_ROM: Range = Range(0x4000, 0x7FFF);

    pub const VIDEO_RAM: Range = Range(0x8000, 0x9FFF);

    pub const CHARACTER_DATA: Range = Range(0x8000, 0x97FF);

    pub const BG_DISPLAY_DATA_1: Range = Range(0x9800, 0x9BFF);

    pub const BG_DISPLAY_DATA_2: Range = Range(0x9C00, 0x9FFF);    

    pub const SWITCHABLE_RAM: Range = Range(0xA000, 0xBFFF);

    pub const INTERNAL_RAM: Range = Range(0xC000, 0xDFFF);

    pub const ECHO_INTERNAL_RAM: Range = Range(0xE000, 0xFDFF);

    pub const SPRITE_ATTRIB_MEMORY: Range = Range(0xFE00, 0xFE9F);

    pub const NOT_USABLE_1: Range = Range(0xFEA0, 0xFEFF);

    pub const IO: Range = Range(0xFF00, 0xFF4B);

    pub const NOT_USABLE_2: Range = Range(0xFF4C, 0xFF7F);

    pub const HIGH_INTERNAL_RAM: Range = Range(0xFF80, 0xFFFE);
}

pub struct Bus {
    mbc: Box<::mbc::MBC+'static>,

    clock: Clock,

    sound_channel_1: Sound,
    sound_channel_2: Sound,
    sound_channel_3: Sound,
    sound_channel_4: Sound,

    pub gui: Gui,

    ie: InterruptEnable,
    ifl: InterruptFlag,

    joypad: Joypad,

    serial: Serial,
}

impl Bus {
    pub fn new(mbc: Box<::mbc::MBC+'static>) -> Bus {
        Bus {
            mbc,
            clock: Clock::new(),

            sound_channel_1: Sound::new(),
            sound_channel_2: Sound::new(),
            sound_channel_3: Sound::new(),
            sound_channel_4: Sound::new(),

            gui: Gui::new(),

            ie: InterruptEnable::new(),
            ifl: InterruptFlag::new(),

            joypad: Joypad::new(),

            serial: Serial::new(),
        }
    }

    pub fn add_to_clock(&mut self, value: u16) {
        self.clock.counter = self.clock.counter.wrapping_add(value);
    }

    pub fn load(&self, addr: u16) -> u8 {
        if let Some(offset) = map::ROM.contains(addr) {
            return self.mbc.readrom(offset)
        }

        if let Some(offset) = map::SWITCHABLE_ROM.contains(addr) {
            return self.mbc.readrom(offset)
        }

        if let Some(offset) = map::VIDEO_RAM.contains(addr) {
            
            if let Some(offset) = map::CHARACTER_DATA.contains(addr) {
                panic!("load CHARACTER_DATA");
            }

            if let Some(offset) = map::BG_DISPLAY_DATA_1.contains(addr) {
                return self.gui.load_bg_display_data_1(addr);
            }

            if let Some(offset) = map::BG_DISPLAY_DATA_2.contains(addr) {
                return self.gui.load_bg_display_data_2(addr);       
            }
        }

        if let Some(offset) = map::SPRITE_ATTRIB_MEMORY.contains(addr) {
            return self.gui.load_sprite(offset);
        }

        if let Some(offset) = map::SWITCHABLE_RAM.contains(addr) {
            return self.mbc.readram(offset)
        }

        if let Some(offset) = map::INTERNAL_RAM.contains(addr) {
            panic!("load INTERNAL_RAM");
            //return self.ram.load(offset);
        }

        if let Some(offset) = map::ECHO_INTERNAL_RAM.contains(addr) {
            panic!("load ECHO_INTERNAL_RAM");
            //return self.ram.load(offset);
        }

        if let Some(offset) = map::HIGH_INTERNAL_RAM.contains(addr) {
            panic!("load HIGH_INTERNAL_RAM");
            //return self.ram.load(offset);
        }

        if let Some(offset) = map::NOT_USABLE_1.contains(addr) {
            return 0;
        }

        if let Some(offset) = map::NOT_USABLE_2.contains(addr) {
            return 0;
        }

        if let Some(offset) = map::IO.contains(addr) {
            match addr {
                0xFF00 => {
                    return self.joypad.get_keys();
                }
                0xFF01 => {
                    return self.serial.data;
                }
                0xFF02 => {
                    return self.serial.control;
                }
                0xFF03 => {
                    return 0;
                }
                0xFF04 => {
                    return self.clock.divider;
                }
                0xFF05 => {
                    return self.clock.counter as u8;
                }
                0xFF06 => {
                    return self.clock.divider;
                }
                0xFF07 => {
                    let status = self.clock.enabled as u8;
                    let value = match self.clock.mode {
                        4096 => 0x00,
                        262144 => 0x01,
                        65536 => 0x10,
                        16384 => 0x11,
                        _ => unreachable!(),
                    };
                    return (status << 2) | value;
                }
                0xFF08 | 0xFF09 | 0xFF0A | 0xFF0B | 0xFF0C | 0xFF0D | 0xFF0E => {
                    return 0;
                }
                0xFF0F => {
                    return self.ifl.get_data();
                }
                0xFF10 => {
                    let time = match self.sound_channel_1.sweep_time.round() as i32 {
                        0 => 0b000,
                        8 => 0b001,
                        16 => 0b010,
                        23 => 0b011,
                        31 => 0b100,
                        39 => 0b101,
                        47 => 0b110,
                        55 => 0b111,
                        _ => unreachable!(),
                    };

                    return time << 4 | (self.sound_channel_1.sweep_mode as u8) << 3
                        | self.sound_channel_1.shift as u8;
                }
                0xFF11 => {
                    let pattern = match self.sound_channel_1.wave_pattern.round() as i32 {
                        13 => 0b00,
                        25 => 0b01,
                        50 => 0b10,
                        75 => 0b11,
                        _ => unreachable!(),
                    };

                    return pattern << 6 | self.sound_channel_1.length as u8;
                }
                0xFF12 => {
                    return (self.sound_channel_1.initial_volume << 4
                        | (self.sound_channel_1.direction as u16) << 3
                        | self.sound_channel_1.sweeps as u16) as u8;
                }
                0xFF13 | 0xFF15 | 0xFF18 => {
                    return 0;
                }
                0xFF14 => {
                    return self.sound_channel_1.initial << 7 | self.sound_channel_1.counter << 6
                        | self.sound_channel_1.frequency;
                }
                0xFF16 => {
                    let pattern = match self.sound_channel_2.wave_pattern.round() as i32 {
                        13 => 0b00,
                        25 => 0b01,
                        50 => 0b10,
                        75 => 0b11,
                        _ => unreachable!(),
                    };

                    return (pattern << 6 | self.sound_channel_2.length) as u8;
                }
                0xFF17 => {
                    return (self.sound_channel_2.initial_volume << 4
                        | (self.sound_channel_2.direction as u16) << 3
                        | self.sound_channel_2.sweeps as u16) as u8;
                }
                0xFF19 => {
                    return self.sound_channel_2.initial << 7 | self.sound_channel_2.counter << 6
                        | self.sound_channel_2.frequency;
                }
                0xFF1A => {
                    return self.sound_channel_3.enable as u8;
                }
                0xFF1B => {
                    return self.sound_channel_3.length as u8;
                }
                0xFF1C => {
                    let volume = match self.sound_channel_3.volume {
                        0 => 0b00,
                        100 => 0b01,
                        50 => 0b10,
                        25 => 0b11,
                        _ => unreachable!(),
                    };

                    return volume;
                }
                0xFF1D | 0xFF1F => return 0,
                0xFF1E => {
                    return self.sound_channel_3.initial << 7 | self.sound_channel_3.counter << 6
                        | self.sound_channel_3.frequency;
                }
                0xFF20 => {
                    return self.sound_channel_4.length as u8;
                }
                0xFF40 => {
                    let window_tile = match self.gui.window_tile_map {
                        0x9800 => 0b0,
                        0x9C00 => 0b1,
                        _ => unreachable!(),
                    };

                    let bg_window_tile = match self.gui.bg_window_tile_map {
                        0x8800 => 0b0,
                        0x8000 => 0b1,
                        _ => unreachable!(),
                    };

                    let bg_tile = match self.gui.window_tile_map {
                        0x9800 => 0b0,
                        0x9C00 => 0b1,
                        _ => unreachable!(),
                    };

                    let obj_size = match self.gui.sprite_size {
                        MIN_SPRITE_SIZE => 0b0,
                        MAX_SPRITE_SIZE => 0b1,
                        _ => unreachable!(),
                    };

                    return (self.gui.lcd_display as u8) << 7 | window_tile << 6
                        | (self.gui.window_display as u8) << 5
                        | bg_window_tile << 4 | bg_tile << 3
                        | obj_size << 2
                        | (self.gui.sprite_display as u8) << 1
                        | self.gui.bg_display as u8;
                }
                0xFF41 => {
                    return 0x80 | self.gui.lyc << 6 | self.gui.mode2 << 5 | self.gui.mode1 << 4 | self.gui.mode0 << 3 | self.gui.coincidence << 2 | self.gui.mode_flag;
                }
                0xFF42 => {
                    return self.gui.scroll_y;
                }
                0xFF43 => {
                    return self.gui.scroll_x;
                }
                0xFF44 => {
                    return self.gui.line;
                }
                0xFF45 => {
                    return self.gui.lyc;
                }
                0xFF47 => {
                    return (self.gui.pallete_base[Color::Black as usize] << 6 | self.gui.pallete_base[Color::DarkGray as usize] << 4 | self.gui.pallete_base[Color::LightGray as usize] << 2 | self.gui.pallete_base[Color::White as usize]) as u8;
                }
                0xFF48 => {
                    // WARNING: 0-1 bytes - 0b11 because of transperency flag
                    return (self.gui.pallete_0[Color::Black as usize] << 6 | self.gui.pallete_0[Color::DarkGray as usize] << 4 | self.gui.pallete_0[Color::LightGray as usize] << 2 | 0b11) as u8;
                }
                0xFF49 => {
                    // WARNING: 0-1 bytes - 0b11 because of transperency flag
                    return (self.gui.pallete_1[Color::Black as usize] << 6 | self.gui.pallete_1[Color::DarkGray as usize] << 4 | self.gui.pallete_1[Color::LightGray as usize] << 2 | 0b11) as u8;
                }
                0xFF4A => {
                    return self.gui.window_y;
                }
                0xFF4B => {
                    return self.gui.window_x;
                }
                _ => panic!("Unimplemented load IO register {:#x}", addr),
            }
        }

        if addr == 0xFFFF {
            let value: u8 = ((self.ie.joypad as u8) << 4) | ((self.ie.serial as u8) << 3)
                | ((self.ie.timer as u8) << 2)
                | ((self.ie.lcd_stat as u8) << 1)
                | self.ie.v_blank as u8;
            return value;
        }

        panic!("Unhandled load 8bit address {:#x}", addr);
    }

    pub fn store16(&mut self, addr: u16, value: u16) {
        // TODO: Probably incorrect. Inverse needed.
        self.store(addr, (value >> 8) as u8);
        self.store(addr + 1, (value & 0xFF) as u8);
    }

    pub fn store(&mut self, addr: u16, value: u8) {
        if let Some(offset) = map::ROM.contains(addr) {
            println!("Unexpected writing to ROM: addr: {:x} value: {:x}", offset, value);
        }

        if let Some(offset) = map::VIDEO_RAM.contains(addr) {
            
            if let Some(offset) = map::CHARACTER_DATA.contains(addr) {
                panic!("store CHARACTER_DATA");
            }

            if let Some(offset) = map::BG_DISPLAY_DATA_1.contains(addr) {
                return self.gui.store_bg_display_data_1(addr, value);         
            }

            if let Some(offset) = map::BG_DISPLAY_DATA_2.contains(addr) {
                return self.gui.store_bg_display_data_2(addr, value);          
            }
        }

        if let Some(offset) = map::HIGH_INTERNAL_RAM.contains(addr) {
            panic!("store HIGH_INTERNAL_RAM");
            //return self.ram.store(offset, value);
        }

        if let Some(offset) = map::SWITCHABLE_ROM.contains(addr) {
            println!("Unexpected writing to SWITCHABLE_ROM: addr: {:x} value: {:x}", offset, value);
        }

        if let Some(offset) = map::IO.contains(addr) {
            match addr {
                0xFF01 => {
                    return self.serial.data = value;
                }
                0xFF02 => {
                    self.serial.transfer_flag = (value >> 7) & 0b1 == 1;
                    self.serial.clock = value & 0b1 == 1;
                    return self.serial.control = value;
                }
                0xFF03 => {
                    return;
                }
                0xFF04 => {
                    return self.clock.divider = 0;
                }
                0xFF05 => {
                    let step = (self.clock.mode / 1024) as u16;

                    if step + self.clock.counter > 0xFF {
                        self.clock.counter = self.clock.divider as u16;
                    // TODO: Probably needed implement Interrupt FF0F and set bit 2 inside as ENABLED
                    // More description: When that interrupt is enabled, then the CPU will execute it by calling the timer interrupt vector at 0050h.
                    } else {
                        self.clock.counter = self.clock.counter.wrapping_add(step);
                    }
                    return;
                }
                0xFF06 => {
                    return self.clock.divider = value;
                }
                0xFF07 => {
                    let status = value & 0x4 == 1;
                    self.clock.enabled = status;
                    self.clock.mode = match value {
                        0x00 => 4096,
                        0x01 => 262144,
                        0x10 => 65536,
                        0x11 => 16384,
                        _ => unreachable!(),
                    };
                    return;
                }
                0xFF0F => {
                    self.ifl.v_blank = value & 0b1 == 1;
                    self.ifl.lcd_stat = (value >> 1) & 0b1 == 1;
                    self.ifl.timer = (value >> 2) & 0b1 == 1;
                    self.ifl.serial = (value >> 3) & 0b1 == 1;
                    self.ifl.joypad = (value >> 4) & 0b1 == 1;
                    return;
                }
                0xFF10 => {
                    let time = (value >> 4) & 0b111;
                    self.sound_channel_1.sweep_time = match time {
                        0b000 => 0.0 as f32,
                        0b001 => 7.8 as f32,
                        0b010 => 15.6 as f32,
                        0b011 => 23.4 as f32,
                        0b100 => 31.3 as f32,
                        0b101 => 39.1 as f32,
                        0b110 => 46.9 as f32,
                        0b111 => 54.7 as f32,
                        _ => unreachable!(),
                    };
                    self.sound_channel_1.sweep_mode = ((value & 0xf) as u8) == 1;
                    self.sound_channel_1.shift = (value & 0b111) as u32;

                    return;
                }
                0xFF11 => {
                    // TODO: Probably needed implement manual wave duty pattern.
                    let pattern = (value >> 6) & 0b11;
                    self.sound_channel_1.wave_pattern = match pattern {
                        0b00 => 12.5 as f32,
                        0b01 => 25.0 as f32,
                        0b10 => 50.0 as f32,
                        0b11 => 75.0 as f32,
                        _ => unreachable!(),
                    };

                    let t1 = (value & 0x3f) as u32;
                    self.sound_channel_1.length = (64 - t1) * (1 / 256);

                    return;
                }
                0xFF12 => {
                    self.sound_channel_1.initial_volume = (value >> 4) as u16;
                    self.sound_channel_1.direction = (value >> 3) & 0b1 == 1;
                    self.sound_channel_1.sweeps = value & 0x4;

                    return;
                }
                0xFF14 => {
                    self.sound_channel_1.initial = (value >> 7) & 0b1;
                    self.sound_channel_1.counter = (value >> 6) & 0b1;
                    self.sound_channel_1.frequency = value & 0b111;

                    return;
                }
                0xFF16 => {
                    // TODO: Probably needed implement manual wave duty pattern.
                    let pattern = (value >> 6) & 0b11;
                    self.sound_channel_2.wave_pattern = match pattern {
                        0b00 => 12.5 as f32,
                        0b01 => 25.0 as f32,
                        0b10 => 50.0 as f32,
                        0b11 => 75.0 as f32,
                        _ => unreachable!(),
                    };

                    let t1 = (value & 0x3f) as u32;
                    self.sound_channel_2.length = (64 - t1) * (1 / 256);

                    return;
                }
                0xFF17 => {
                    self.sound_channel_2.initial_volume = (value >> 4) as u16;
                    self.sound_channel_2.direction = (value >> 3) & 0b1 == 1;
                    self.sound_channel_2.sweeps = value & 0x4;

                    return;
                }
                0xFF19 => {
                    self.sound_channel_2.initial = (value >> 7) & 0b1;
                    self.sound_channel_2.counter = (value >> 6) & 0b1;
                    self.sound_channel_2.frequency = value & 0b111;

                    return;
                }
                0xFF1A => {
                    self.sound_channel_3.enable = (value >> 7) & 0b1 == 1;

                    return;
                }
                0xFF1B => {
                    self.sound_channel_3.length = ((256 as u32 - value as u32) * (1 / 256)) as u32;

                    return;
                }
                0xFF1C => {
                    let volume = (value >> 5) & 0b11;
                    self.sound_channel_3.volume = match volume {
                        0b00 => 0,
                        0b01 => 100,
                        0b10 => 50,
                        0b11 => 25,
                        _ => unreachable!(),
                    };

                    return;
                }
                0xFF1E => {
                    self.sound_channel_3.initial = (value >> 7) & 0b1;
                    self.sound_channel_3.counter = (value >> 6) & 0b1;
                    self.sound_channel_3.frequency = value & 0b111;

                    return;
                }
                0xFF20 => {
                    let t1 = (value & 0x3f) as u32;
                    self.sound_channel_4.length = (64 - t1) * (1 / 256);

                    return;
                }
                0xFF21 => {
                    self.sound_channel_4.initial_volume = (value >> 4) as u16;
                    self.sound_channel_4.direction = (value >> 3) & 0b1 == 1;
                    self.sound_channel_4.sweeps = value & 0x4;

                    return;
                }
                0xFF22 => {
                    self.sound_channel_4.shift_clock = (value >> 4) as u32;
                    self.sound_channel_4.width = ((value >> 3) & 0b1) as u32;
                    self.sound_channel_4.ratio = (value & 0b111) as u32;

                    return;
                }
                0xFF23 => {
                    self.sound_channel_4.initial = (value >> 7) & 0b1;
                    self.sound_channel_4.counter = (value >> 6) & 0b1;

                    return;
                }
                0xFF24 => {
                    self.sound_channel_2.enable = (value >> 7) & 0b1 == 1;
                    self.sound_channel_2.volume = ((value >> 4) & 0b111) as u32;

                    self.sound_channel_1.enable = (value >> 3) & 0b1 == 1;
                    self.sound_channel_1.volume = (value & 0b111) as u32;

                    return;
                }
                0xFF25 => {
                    // TODO: Probably incorrect implementation.
                    self.sound_channel_2.data[7] = (value >> 7) & 0b1;
                    self.sound_channel_2.data[6] = (value >> 6) & 0b1;
                    self.sound_channel_2.data[5] = (value >> 5) & 0b1;
                    self.sound_channel_2.data[4] = (value >> 4) & 0b1;

                    self.sound_channel_1.data[3] = (value >> 3) & 0b1;
                    self.sound_channel_1.data[2] = (value >> 2) & 0b1;
                    self.sound_channel_1.data[1] = (value >> 1) & 0b1;
                    self.sound_channel_1.data[0] = value & 0b1;

                    return;
                }
                0xFF26 => {
                    // TODO: Probably incorrect implementation.
                    let all = (value >> 7) & 0b1 == 1;

                    if all {
                        self.sound_channel_1.enable = true;
                        self.sound_channel_2.enable = true;
                        self.sound_channel_3.enable = true;
                        self.sound_channel_4.enable = true;
                    } else {
                        self.sound_channel_1.enable = false;
                        self.sound_channel_2.enable = false;
                        self.sound_channel_3.enable = false;
                        self.sound_channel_4.enable = false;
                    }

                    return;
                }
                0xFF40 => {
                    self.gui.lcd_display = (value >> 7) & 0b1 == 1;

                    let window_tile = (value >> 6) & 0b1;

                    self.gui.window_tile_map = match window_tile {
                        0b0 => 0x9800,
                        0b1 => 0x9C00,
                        _ => unreachable!(),
                    };

                    self.gui.window_display = (value >> 5) & 0b1 == 1;

                    let bg_window_tile = (value >> 4) & 0b1;

                    self.gui.bg_window_tile_map = match bg_window_tile {
                        0b0 => 0x8800,
                        0b1 => 0x8000,
                        _ => unreachable!(),
                    };

                    let bg_tile = (value >> 3) & 0b1;

                    self.gui.window_tile_map = match bg_tile {
                        0b0 => 0x9800,
                        0b1 => 0x9C00,
                        _ => unreachable!(),
                    };

                    let obj_size = (value >> 2) & 0b1;

                    self.gui.sprite_size = match obj_size {
                        0b0 => MIN_SPRITE_SIZE,
                        0b1 => MAX_SPRITE_SIZE,
                        _ => unreachable!(),
                    };

                    self.gui.sprite_display = (value >> 1) & 0b1 == 1;

                    self.gui.bg_display = value & 0b1 == 1;

                    return;
                }
                0xFF41 => {
                    self.gui.lyc = (value >> 6) & 0b1;

                    self.gui.mode2 = (value >> 5) & 0b1;
                    self.gui.mode1 = (value >> 4) & 0b1;
                    self.gui.mode0 = (value >> 3) & 0b1;
                    return;
                }
                0xFF42 => {
                    self.gui.scroll_y = value;
                    return;
                }
                0xFF43 => {
                    self.gui.scroll_x = value;
                    return;
                }
                0xFF45 => {
                    self.gui.lyc = value;
                    return;
                }
                0xFF47 => {
                    self.gui.pallete_base[Color::Black as usize] = ((value >> 6) & 0b11) as u16;
                    self.gui.pallete_base[Color::DarkGray as usize] = ((value >> 4) & 0b11) as u16;
                    self.gui.pallete_base[Color::LightGray as usize] = ((value >> 2) & 0b11) as u16;
                    self.gui.pallete_base[Color::White as usize] = (value & 0b11) as u16;
                    return;
                }
                0xFF48 => {
                    self.gui.pallete_0[Color::Black as usize] = ((value >> 6) & 0b11) as u16;
                    self.gui.pallete_0[Color::DarkGray as usize] = ((value >> 4) & 0b11) as u16;
                    self.gui.pallete_0[Color::LightGray as usize] = ((value >> 2) & 0b11) as u16;
                    return;
                }
                0xFF49 => {
                    self.gui.pallete_1[Color::Black as usize] = ((value >> 6) & 0b11) as u16;
                    self.gui.pallete_1[Color::DarkGray as usize] = ((value >> 4) & 0b11) as u16;
                    self.gui.pallete_1[Color::LightGray as usize] = ((value >> 2) & 0b11) as u16;
                    return;
                }
                0xFF4A => {
                    if value > 0 || value <= 143 {
                        self.gui.window_y = value;
                    } else {
                        panic!("broken window y size");
                    }
                    return;
                }
                0xFF4B => {
                    if value > 0 || value <= 166 {
                        self.gui.window_x = value;
                    } else {
                        panic!("broken window x size");
                    }
                    return;
                }
                _ => panic!("Unimplemented store IO register {:#x}", addr),
            }
        }

        if let Some(offset) = map::ECHO_INTERNAL_RAM.contains(addr) {
            panic!("store ECHO_INTERNAL_RAM");
            // self.rom.store(offset - 0x2000, value); // internal
            // return self.rom.store(offset, value); //echo
        }

        if let Some(offset) = map::INTERNAL_RAM.contains(addr) {
            panic!("store INTERNAL_RAM");
            // self.rom.store(offset + 0x2000, value); //echo
            // return self.rom.store(offset, value); //internal
        }

        if addr == 0xFFFF {
            self.ie.v_blank = value & 0b1 == 1;
            self.ie.lcd_stat = (value >> 1) & 0b1 == 1;
            self.ie.timer = (value >> 2) & 0b1 == 1;
            self.ie.serial = (value >> 3) & 0b1 == 1;
            self.ie.joypad = (value >> 4) & 0b1 == 1;
            return;
        }

        panic!("Unhandled store 8bit address {:#x}", addr);
    }
}

struct InterruptEnable {
    v_blank: bool,
    lcd_stat: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl InterruptEnable {
    pub fn new() -> InterruptEnable {
        InterruptEnable {
            v_blank: false,
            lcd_stat: false,
            timer: false,
            serial: false,
            joypad: false,
        }
    }

    pub fn get_data(&self) -> u8 {
        return (self.joypad as u8) << 4 | (self.serial as u8) << 3 | (self.timer as u8) << 2
            | (self.lcd_stat as u8) << 1 | (self.v_blank as u8);
    }
}

struct InterruptFlag {
    v_blank: bool,
    lcd_stat: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl InterruptFlag {
    pub fn new() -> InterruptFlag {
        InterruptFlag {
            v_blank: true,
            lcd_stat: false,
            timer: false,
            serial: false,
            joypad: false,
        }
    }

    pub fn get_data(&self) -> u8 {
        return (self.joypad as u8) << 4 | (self.serial as u8) << 3 | (self.timer as u8) << 2
            | (self.lcd_stat as u8) << 1 | (self.v_blank as u8);
    }
}
