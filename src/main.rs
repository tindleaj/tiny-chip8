use std::env;
use std::fs;

use tiny_chip8::*;

fn main() {
    let rom = fs::read("roms/test_opcode.ch8").expect("Problem reading file");
    let mut chip8 = Chip8::new();

    chip8.load_rom(&rom);

    loop {
        let mut line = 0;

        chip8.execute();
        println!("{:?} current_op:{:x?} {:x?}", chip8.debug_info(), chip8.current_op.0, chip8.current_op.1);
        println!("{:?}", chip8.vram);

        line += 1;
    }
}
