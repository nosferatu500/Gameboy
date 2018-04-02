extern crate clap;
extern crate sdl2;

#[macro_use]
extern crate nom;

use clap::{App, Arg};

use std::process;

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
mod debugger;

use rom::Rom;
use bus::Bus;
use cpu::Cpu;

use debugger::Debugger;

fn main() {
    let matches = App::new("Gameboy Emulator")
        .version("0.1")
        .author("Vitaly Shvetsov <nosferatu2995@mail.ru>")
        .about("Research GB emulation process")
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("Sets the rom file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .help("Write cargo run <file> -- -d for enable debug mode"),
        )
        .arg(
            Arg::with_name("log")
                .short("l")
                .help("Write cargo run <file> -- -l for enable log mode"),
        )
        .get_matches();

    let rom_file = matches.value_of("file").unwrap();

    let rom = Rom::new(&rom_file).unwrap();

    let bus = Bus::new(rom);

    let mut cpu = Cpu::new(bus);

    cpu.power_up();

    if matches.is_present("log") {
        cpu.enable_log();
    }

    if matches.is_present("debug") {
        cpu.enable_log();
        let mut debugger = Debugger::new(cpu);
        debugger.run();
    } else {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Gameboy Emulator by Vitaly Shvetsov", 160 * 5, 144 * 5)
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

        cpu.test_draw();

        loop {
            cpu.update_ime();

            cpu.run_next_instruction();

            let _ = renderer.set_draw_color(white);
            let _ = renderer.clear();

            for x in 0..160 {
                for y in 0..144 {
                    let data = &cpu.get_data_gui(x, y);

                    let c1 = (data >> 16) & 0xFF as u32;
                    let c2 = (data >> 8) & 0xFF as u32;
                    let c3 = data & 0xFF as u32;

                    if c1 != 0x00  as u32 || c2 != 0x00  as u32 || c3 != 0x00  as u32 {
                        let new_color = sdl2::pixels::Color::RGB(c1 as u8, c2 as u8, c3 as u8);

                        let x_pos = (x * 5) as i32;
                        let y_pos = (y * 5) as i32;
                        
                        rect.set_y(y_pos);
                        rect.set_x(x_pos);
                        
                        let _ = renderer.fill_rect(rect);
                        let _ = renderer.set_draw_color(new_color);
                    }
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
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}
