pub const MAX_SPRITES: u32 = 40;
pub const MAX_LINE: u32 = 10;

pub const MAX_SPRITE_SIZE: u32 = 16;
pub const MIN_SPRITE_SIZE: u32 = 8;

#[derive(Clone, Copy)]
pub enum Color {
    //TODO: Probably we must using A-RGB format where A - 0xFF
    White = 0b00, // 0xFFFFFF
    LightGray = 0b01, // 0xAAAAAA
    DarkGray = 0b10, // 0x555555
    Black = 0b11, // 0x000000
}

// lcdc 91
// stat 85
// cnt  28
// if   E1

pub struct OAM {
    coord_x: u8,
    coord_y: u8,

    priority: u8,

    flip_h: bool,
    flip_v: bool,

    size_x: u32,
    size_y: u32,
} 

pub struct Gui {
    pub data: [[[u8; 3]; 144]; 160],
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

    pub mode2: u8,
    pub mode1: u8,
    pub mode0: u8,

    pub line: u8,

    bg_display_data_1: [u8; 1024],
    bg_display_data_2: [u8; 1024],
}

impl Gui {
    pub fn new() -> Gui {
        Gui {
            data: [[[0; 3]; 144]; 160],
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

            mode2: 0,
            mode1: 0,
            mode0: 0,

            line: 0,

            bg_display_data_1: [0; 1024],
            bg_display_data_2: [0; 1024],
        }
    }

    pub fn test_draw(&mut self) {
        //Initialize. 
        //It's weird, but without pixel on 0:0, another pixels have not detected status.
        self.data[0][0][0] = 0xFF;
        self.data[0][0][1] = 0x00; 
        self.data[0][0][2] = 0x00;

        self.data[160/2][144/2][0] = 0xFF;
        self.data[160/2][144/2][1] = 0x00; 
        self.data[160/2][144/2][2] = 0x00;
    }

    pub fn get_data(&self, x: usize, y: usize) -> u32 {
        let c1 = self.data[x][y][0] as u32;
        let c2 = self.data[x][y][1] as u32;
        let c3 = self.data[x][y][2] as u32;

        c1 << 16 | c2 << 8 | c3
    }

    pub fn store_bg_display_data_1(&mut self, address: u16, value: u8) {
        self.bg_display_data_1[address as usize] = value;
    }

    pub fn load_bg_display_data_1(&self, address: u16) -> u8 {
        self.bg_display_data_1[address as usize]
    }

    pub fn store_bg_display_data_2(&mut self, address: u16, value: u8) {
        self.bg_display_data_2[address as usize] = value;
    }

    pub fn load_bg_display_data_2(&self, address: u16) -> u8 {
        self.bg_display_data_2[address as usize]
    }

    pub fn load_sprite(&self, address: u16) -> u8 {
        self.sprite_attrib[address as usize]
    }

    pub fn store_sprite(&mut self, address: u16, value: u8) {
        self.sprite_attrib[address as usize] = value;
    }
}
