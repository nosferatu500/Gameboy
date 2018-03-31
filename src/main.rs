extern crate sdl2;

use std::process;
use std::env::args;
use std::time::{Duration, Instant};

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod rom;
mod bus;
mod cpu;

mod register;
mod clock;
mod sound;
mod gui;
mod ram;
mod joypad;
mod serial;

use rom::Rom;
use bus::Bus;
use cpu::Cpu;

fn main() {
    let rom_file = args().nth(1).unwrap();

    let rom = Rom::new(&rom_file).unwrap();

    let bus = Bus::new(rom);

    let mut cpu = Cpu::new(bus);

    cpu.power_up();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("VirtualBoy Emulator by Vitaly Shvetsov", 160 * 5, 144 * 5)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let mut rect = Rect::new(5, 5, 5, 5);

    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    let mut events = sdl_context.event_pump().unwrap();

    loop {
        cpu.update_ime();

        cpu.run_next_instruction();

        let _ = renderer.set_draw_color(black);
        let _ = renderer.clear();

        for x in 0..160 {
            for y in 0..144 {
                    //let data = bus.gui.get_data(x, y);
                   // if data {
                        let x_pos = (x * 5) as i32;
                        let y_pos = (y * 5) as i32;
                        rect.set_y(y_pos);
                        rect.set_x(x_pos);
                        let _ = renderer.fill_rect(rect);
                        let _ = renderer.set_draw_color(white);
                //}
            }
        }
        let _ = renderer.present();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    process::exit(1);
                }

                _ => {}
            }
        }
    }
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}
