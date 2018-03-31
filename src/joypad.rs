pub struct Joypad {
    is_direction: bool,

    down: bool,
    up: bool,
    left: bool,
    right: bool,

    start: bool,
    select: bool,
    b: bool,
    a: bool,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            is_direction: false, // false - selected

            down: true, // false - pressed
            up: true,
            left: true,
            right: true,

            start: true,
            select: true,
            b: true,
            a: true,
        }
    }

    pub fn get_keys(&self) -> u8 {
        if !self.is_direction {
            return (self.down as u8) << 3 | (self.up as u8) << 2 | (self.left as u8) << 1
                | (self.right as u8);
        } else {
            return (self.start as u8) << 3 | (self.select as u8) << 2 | (self.b as u8) << 1
                | (self.a as u8);
        }
    }
}
