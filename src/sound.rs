pub struct Sound {
    pub sweep_time: f32,
    pub sweep_mode: bool, // 0 - Increase; 1 - Decrease;
    pub shift: u32,

    pub wave_pattern: f32,
    pub length: u32,

    pub initial_volume: u16,
    pub direction: bool, // 0 - Increase; 1 - Decrease;
    pub sweeps: u8,

    pub initial: u8,
    pub counter: u8,
    pub frequency: u8,

    pub enable: bool,

    pub volume: u32,

    pub shift_clock: u32,
    pub width: u32,
    pub ratio: u32,

    pub data: [u8; 32],
}

impl Sound {
    pub fn new() -> Sound {
        Sound {
            sweep_time: 0.0,
            sweep_mode: false,
            shift: 0,

            wave_pattern: 50.0,
            length: 0,

            initial_volume: 0,
            direction: false,
            sweeps: 0,

            initial: 0,
            counter: 0,
            frequency: 0,

            enable: false,

            volume: 0,

            shift_clock: 0,
            width: 0,
            ratio: 0,

            data: [0; 32],
        }
    }
}
