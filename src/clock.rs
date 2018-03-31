pub struct Clock {
    pub divider: u8,
    pub counter: u16,
    pub mode: u32,

    pub enabled: bool,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            divider: 0,
            counter: 0,
            mode: 0,

            enabled: false,
        }
    }
}
