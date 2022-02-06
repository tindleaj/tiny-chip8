use crate::*;

extern crate std;

#[test]
fn test_load_rom() {
    let mut c = Chip8::new();
    let mut rom = [1; 4096];

    rom[0] = 255;
    rom[1] = 2;
    rom[2] = 3;
    c.load_rom(&rom);

    assert_eq!(c.memory[0x200], 255);
    assert_eq!(c.memory[0x201], 2);
    assert_eq!(c.memory[0x202], 3);
}

#[test]
fn test_first_512_off_limits() {
    let mut c = Chip8::new();
    let rom = [1; 4096];

    c.load_rom(&rom);

    for i in 0x000..0x1FF {
        assert_eq!(c.memory[i], 0);
    }
}

#[test]
fn test_to_nybbles() {
    let op: (u8, u8) = (0x12, 0x4e);

    let nybbles = to_nybbles(op);

    assert_eq!(nybbles.0, 0x01);
    assert_eq!(nybbles.1, 0x02);
    assert_eq!(nybbles.2, 0x04);
    assert_eq!(nybbles.3, 0x0e);
}

#[test]
fn test_op_1nnn() {
    let mut c = Chip8::new();
    c.run_op((0x12, 0x4e));

    assert_eq!(c.pc, 0x024e);
}

#[test]
fn test_op_6xnn() {
    let mut c = Chip8::new();
    c.run_op((0x61, 0x4e));

    assert_eq!(c.v[1], 0x4e);
}

#[test]
fn test_op_annn() {
    let mut c = Chip8::new();
    c.run_op((0xa1, 0x4e));

    assert_eq!(c.i, 0x14e);
}

#[test]
fn test_op_dxyn() {
    let mut c = Chip8::new();
    c.memory[0] = 0b11111000;
    c.v[0] = 0;
    c.v[1] = 0;
    c.i = 0;

    c.run_op((0xd0, 0x11));

    assert_eq!(c.vram[0][0..=7], [1, 1, 1, 1, 1, 0, 0, 0]);
}

#[test]
fn test_op_dxyn_x_wrap() {
    let mut c = Chip8::new();
    c.memory[0] = 0b11111111;
    c.v[0] = 59;
    c.v[1] = 0;
    c.i = 0;

    c.run_op((0xd0, 0x11));

    assert_eq!(c.vram[0][59..=63], [1, 1, 1, 1, 1]);
    assert_eq!(c.vram[0][0..=2], [1, 1, 1]);
}

#[test]
fn test_op_dxyn_y_wrap() {
    let mut c = Chip8::new();
    c.memory[0] = 0b10000000;
    c.memory[1] = 0b10000000;
    c.v[0] = 0;
    c.v[1] = 31;
    c.i = 0;

    c.run_op((0xd0, 0x12));

    assert_eq!(c.vram[31][0], 1);
    assert_eq!(c.vram[0][0], 1);
}

#[test]
fn test_op_3xnn() {
    let mut c = Chip8::new();
    c.run_op((0x30, 0x00));

    assert_eq!(c.pc, 0x204);
}

#[test]
fn test_op_4xnn() {
    let mut c = Chip8::new();
    c.run_op((0x40, 0x01));

    assert_eq!(c.pc, 0x204);
}

#[test]
fn test_op_5xy0() {
    let mut c = Chip8::new();

    c.v[0] = 1;
    c.v[1] = 1;
    c.run_op((0x50, 0x10));

    assert_eq!(c.pc, 0x204);
}

#[test]
fn test_op_7xnn() {
    let mut c = Chip8::new();

    c.run_op((0x70, 0x05));

    assert_eq!(c.v[0], 5);
}

#[test]
fn test_op_9xy0() {
    let mut c = Chip8::new();

    c.v[0] = 1;
    c.v[1] = 2;
    c.run_op((0x90, 0x10));

    assert_eq!(c.pc, 0x204);
}

#[test]
fn test_op_2nnn() {
    let mut c = Chip8::new();

    c.run_op((0x21, 0x11));

    assert_eq!(c.pc, 0x111);
    assert_eq!(c.sp, 1);
    assert_eq!(c.stack[0], 0x202);
}

#[test]
fn test_op_00ee() {
    let mut c = Chip8::new();

    c.run_op((0x21, 0x11)); // go to subroutine
    c.run_op((0x00, 0xee)); // return

    assert_eq!(c.pc, 0x202);
    assert_eq!(c.sp, 0);
}

#[test]
fn test_op_8xy0() {
    let mut c = Chip8::new();
    c.v[0] = 255;
    c.v[1] = 1;

    c.run_op((0x80, 0x10));

    assert_eq!(c.v[0], c.v[1]);
}

#[test]
fn test_op_8xy1() {
    let mut c = Chip8::new();
    c.v[0] = 0b01111110;
    c.v[1] = 0b11000000;

    c.run_op((0x80, 0x11));

    assert_eq!(c.v[0], 0b11111110);
}

#[test]
fn test_op_8xy2() {
    let mut c = Chip8::new();
    c.v[0] = 0b01111111;
    c.v[1] = 0b10000001;

    c.run_op((0x80, 0x12));

    assert_eq!(c.v[0], 0b00000001);
}

#[test]
fn test_op_8xy3() {
    let mut c = Chip8::new();
    c.v[0] = 0b01111111;
    c.v[1] = 0b10000001;

    c.run_op((0x80, 0x13));

    assert_eq!(c.v[0], 0b11111110);
}

#[test]
fn test_op_8xy4() {
    let mut c = Chip8::new();
    c.v[0] = 3;
    c.v[1] = 5;

    c.run_op((0x80, 0x14));

    assert_eq!(c.v[0], 8);
    assert_eq!(c.v[15], 0);
}

#[test]
fn test_op_8xy4_carry() {
    let mut c = Chip8::new();
    c.v[0] = 0b11111111;
    c.v[1] = 0b00000001;

    c.run_op((0x80, 0x14));

    assert_eq!(c.v[0], 0);
    assert_eq!(c.v[15], 1);
}

#[test]
fn test_op_8xy5() {
    let mut c = Chip8::new();
    c.v[0] = 5;
    c.v[1] = 3;

    c.run_op((0x80, 0x15));

    assert_eq!(c.v[0], 2);
    assert_eq!(c.v[15], 1);
}

#[test]
fn test_op_8xy5_borrow() {
    let mut c = Chip8::new();
    c.v[0] = 0b00000000;
    c.v[1] = 0b00000001;

    c.run_op((0x80, 0x15));

    assert_eq!(c.v[0], 0b11111111);
    assert_eq!(c.v[15], 0);
}

#[test]
fn test_op_8xye() {
    let mut c = Chip8::new();
    c.v[0] = 0b00000000;
    c.v[1] = 0b10000001;

    c.run_op((0x80, 0x1e));

    assert_eq!(c.v[0], 0b00000010);
    assert_eq!(c.v[15], 1);
}

#[test]
fn test_op_8xy6() {
    let mut c = Chip8::new();
    c.v[0] = 0b00000000;
    c.v[1] = 0b10000001;

    c.run_op((0x80, 0x16));

    assert_eq!(c.v[0], 0b01000000);
    assert_eq!(c.v[15], 1);
}

#[test]
fn test_op_fx55() {
    let mut c = Chip8::new();
    c.v[0] = 1;
    c.v[1] = 2;

    c.run_op((0xf1, 0x55));

    assert_eq!(c.memory[0], 1);
    assert_eq!(c.memory[1], 2);
    assert_eq!(c.i, 2);
}

#[test]
fn test_op_fx65() {
    let mut c = Chip8::new();
    c.memory[0] = 1;
    c.memory[1] = 2;

    c.run_op((0xf1, 0x65));

    assert_eq!(c.v[0], 1);
    assert_eq!(c.v[1], 2);
    assert_eq!(c.i, 2);
}

#[test]
fn test_op_fx33() {
    let mut c = Chip8::new();
    c.v[0] = 255;

    c.run_op((0xf0, 0x33));

    assert_eq!(c.memory[0], 0b0010);
    assert_eq!(c.memory[1], 0b00101);
    assert_eq!(c.memory[2], 0b00101);
}

#[test]
fn test_op_8xy7() {
    let mut c = Chip8::new();
    c.v[0] = 0b00000001;
    c.v[1] = 0b00000000;

    c.run_op((0x80, 0x17));

    assert_eq!(c.v[0], 0b11111111);
    assert_eq!(c.v[15], 0);
}

#[test]
fn test_op_bnnn() {
    let mut c = Chip8::new();
    c.v[0] = 0b00000010;

    c.run_op((0xb2, 0x00));

    assert_eq!(c.pc, 0x202)

}