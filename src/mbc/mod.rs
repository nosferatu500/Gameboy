use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod mbc0;

// Cartrige types

// 0x0147 - type
// 0x0148 - ROM size
// 0x0149 - RAM size

// 00h ROM ONLY
// 01h ROM+MBC1
// 02h ROM+MBC1+RAM
// 03h ROM+MBC1+RAM+BATT
// 05h ROM+MBC2
// 06h ROM+MBC2+BATTERY
// 08h ROM+RAM                                               
// 09h ROM+RAM+BATTERY
// 0Bh ROM+MMM01
// 0Ch ROM+MMM01+SRAM
// 0Dh ROM+MMM01+SRAM+BATT     
// 0Fh ROM+MBC3+TIMER+BATT     
// 10h ROM+MBC3+TIMER+RAM+BATT
// 11h ROM+MBC3
// 12h ROM+MBC3+RAM
// 13h ROM+MBC3+RAM+BATT
// 19h ROM+MBC5
// 1Ah ROM+MBC5+RAM
// 1Bh ROM+MBC5+RAM+BATT
// 1Ch ROM+MBC5+RUMBLE
// 1Dh ROM+MBC5+RUMBLE+SRAM
// 1Eh ROM+MBC5+RUMBLE+SRAM+BT
// 1Fh Pocket Camera
// FDh Bandai TAMA5
// FEh Hudson HuC-3
// FFh Hudson HuC-1

pub trait MBC {
    fn readrom(&self, address: u16) -> u8;
    
    fn writerom(&mut self, address: u16, value: u8);
    
    fn readram(&self, address: u16) -> u8;
    
    fn writeram(&mut self, address: u16, value: u8);
}

pub fn get_mbc<P: AsRef<Path>>(path: P) -> ::StrResult<Box<MBC+'static>> {
    let mut file = File::open(&path).unwrap();

    let mut buf = Vec::new();

    file.read_to_end(&mut buf).unwrap();
    
    try!(check_checksum(&buf));
    
    match buf[0x147] {
        0x00 => mbc0::MBC0::new(buf).map(|v| Box::new(v) as Box<MBC>),
        _ => { Err("Unsupported MBC type") },
    }
}

fn check_checksum(data: &[u8]) -> ::StrResult<()> {
    let mut value: u8 = 0;
    for i in 0x134 .. 0x14D {
        value = value.wrapping_sub(data[i]).wrapping_sub(1);
    }
    match data[0x14D] == value
    {
        true => Ok(()),
        false => Err("Cartridge checksum is invalid"),
    }
}
