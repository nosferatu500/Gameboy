pub const MAX_SPRITES: u32 = 40;
pub const MAX_LINE: u32 = 10;

pub const MAX_SPRITE_SIZE: u32 = 16;
pub const MIN_SPRITE_SIZE: u32 = 8;

#[derive(Clone, Copy)]
pub enum Color {
    White = 0b00,
    LightGray = 0b01,
    DarkGray = 0b10,
    Black = 0b11,
}

pub struct Gui {
    pub data: [[u8; 160]; 144],
    pub sprite_attrib: [u8; 8 * 1024],

    pub lcd_display: bool,

    pub window_tile_map: u32,

    pub window_display: bool,

    pub bg_window_tile_map: u32,

    pub bg_tile_map: u32,

    pub sprite_size: u32,

    pub sprite_display: bool,

    pub bg_display: bool,

    pub scroll_y: u8,
    pub scroll_x: u8,

    pub lyc: u8,

    pub color: Color,

    pub pallete_base: [u16; 4],

    pub pallete_0: [u16; 4],
    pub pallete_1: [u16; 4],

    pub window_y: u8,
    pub window_x: u8,
}

impl Gui {
    pub fn new() -> Gui {
        Gui {
            data: [[0; 160]; 144],
            sprite_attrib: [0; 8 * 1024],

            lcd_display: false,

            window_tile_map: 0,

            window_display: false,

            bg_window_tile_map: 0,

            bg_tile_map: 0,

            sprite_size: 0,

            sprite_display: false,

            bg_display: false,

            scroll_y: 0,
            scroll_x: 0,

            lyc: 0,

            color: Color::White,

            pallete_base: [0; 4],

            pallete_0: [0; 4],
            pallete_1: [0; 4],

            window_y: 0,
            window_x: 0,
        }
    }

    pub fn get_data(&self, x: usize, y: usize) -> bool {
      self.data[y][x] == 1
    }

    pub fn load(&self, address: u16) -> u8 {
        self.data[(address as usize) % 144][(address as usize) % 160]
    }

    pub fn store(&mut self, address: u16, value: u8) {
        self.data[(address as usize) % 144][(address as usize) % 160] = value;
    }

    pub fn load_sprite(&self, address: u16) -> u8 {
        self.sprite_attrib[address as usize]
    }

    pub fn store_sprite(&mut self, address: u16, value: u8) {
        self.sprite_attrib[address as usize] = value;
    }
}
