#![no_std]

const MEMORY: usize = 4096;
const ROM_LOAD_INDEX: usize = 0x200;
const V_HEIGHT: usize = 32;
const V_WIDTH: usize = 64;

enum Pc {
    Next,
    Jump(usize),
    Skip(usize)
}

#[derive(Debug)]
pub struct Chip8 {
    i: usize,
    v: [u8; 16],
    pub vram: [[u8; V_WIDTH]; V_HEIGHT],
    pub vram_changed: bool,
    pc: usize,
    stack: [usize; 16],
    sp: usize,
    pub memory: [u8; MEMORY],
    pub current_op: (u8, u8)
}

#[derive(Debug)]
pub struct DebugInfo {
    pub i: usize,
    pub v: [u8; 16],
    pub pc: usize,
    pub stack: [usize; 16],
    pub sp: usize,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            i: 0,
            v: [0; 16],
            vram: [[0; V_WIDTH]; V_HEIGHT],
            vram_changed: false,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            memory: [0; 4096],
            current_op: (0, 0)
        }
    }

    pub fn debug_info(&self) -> DebugInfo {
        DebugInfo {
            i: self.i,
            v: self.v,
            pc: self.pc,
            stack: self.stack,
            sp: self.sp,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, ins) in rom.iter().enumerate() {
            let addr = ROM_LOAD_INDEX + i;

            // Safely mutate memory at the address
            if self.memory.get(addr).is_some() {
                self.memory[addr] = *ins;
            }
        }
    }


    pub fn execute(&mut self) {
        let op = self.get_current_op();

        self.run_op(op)
    }

    fn get_current_op(&self) -> (u8, u8) {
        let index = self.pc as usize;

        if self.memory.get(index + 1).is_some() {
            return (self.memory[index], self.memory[index+1])
        }

        (0, 0) // should return no_op here instead
    }

    pub fn run_op(&mut self, opcode: (u8, u8)) {
        let nybbles = to_nybbles(opcode);

        let nnn = u16::from_be_bytes([(opcode.0 & 0x0F), opcode.1]) as usize;
        let nn = opcode.1 as u8;
        let x = nybbles.1 as usize;
        let y = nybbles.2 as usize;
        let n = nybbles.3 as usize;

        self.current_op = opcode;

        let pc_next: Pc = match nybbles {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x00, _, _, _) => self.op_0nnn(),
            (0x01, _,  _,  _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xnn(x, nn),
            (0x04, _, _, _) => self.op_4xnn(x, nn),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xnn(x, nn),
            (0x07, _, _, _) => self.op_7xnn(x, nn),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8xy6(x, y),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8xye(x, y),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_annn(nnn),
            (0x0b, _, _, _) => self.op_bnnn(nnn),
            (0x0c, _, _, _) => self.op_cxnn(x, nn),
            (0x0d, _, _, _) => self.op_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(x),

            (_, _, _, _) => self.no_op()
        };
        
        match pc_next {
            Pc::Jump(addr) => self.pc = addr,
            Pc::Skip(i) => self.pc += (i*2) + 2,
            Pc::Next => self.pc +=2, // opcodes are two bytes long
        }
    }

    // 0NNN - Execute machine language subroutine at address NNN
    fn op_0nnn(&self) -> Pc {
        unimplemented!()
    }

    // 00E0 - Clear the screen
    fn op_00e0(&self) -> Pc {
        unimplemented!()
    }

    // 00EE - Return from subroutine
    fn op_00ee(&mut self) -> Pc {
        self.sp -= 1;
        
        Pc::Jump(self.stack[self.sp])
    }

    // 1NNN - Jump to address NNN
    fn op_1nnn(&self, nnn: usize) -> Pc {
        Pc::Jump(nnn)
    }

    // 2NNN - Execute subroutine starting at address NNN
    fn op_2nnn(&mut self, nnn: usize) -> Pc {
        self.stack[self.sp] = self.pc + 2;
        self.sp += 1;

        Pc::Jump(nnn)
    }

    // 3XNN - Skip the following instruction if the value of register VX equals NN
    fn op_3xnn(&self, x: usize, nn: u8) -> Pc {
        if self.v[x] == nn {
            return Pc::Skip(1)
        }

        Pc::Next
    }

    // 4XNN - Skip the following instruction if the value of register VX is not equal to NN
    fn op_4xnn(&self, x: usize, nn: u8) -> Pc {
        if self.v[x] != nn {
            return Pc::Skip(1)
        }

        Pc::Next
    }

    // 5XY0 - Skip the following instruction if the value of register VX is equal to the value of register VY
    fn op_5xy0(&self, x: usize, y: usize) -> Pc {
        if self.v[x] == self.v[y] {
            return Pc::Skip(1)
        }

        Pc::Next
    }

    // 6XNN - Store number NN in register VX
    fn op_6xnn(&mut self, x: usize, nn: u8) -> Pc {
        self.v[x as usize] = nn;
        Pc::Next
    }

    // 7XNN - Add the value NN to register VX
    fn op_7xnn(&mut self, x: usize, nn: u8) -> Pc {
        self.v[x] = self.v[x].wrapping_add(nn);

        Pc::Next
    }

    // 8XY0 - Store the value of register VY in register VX
    fn op_8xy0(&mut self, x: usize, y: usize) -> Pc {
        self.v[x] = self.v[y];

        Pc::Next
    }

    // 8XY1 - Set VX to VX OR VY
    fn op_8xy1(&mut self, x: usize, y: usize) -> Pc {
        self.v[x] = self.v[y] | self.v[x];

        Pc::Next
    }

    // 8XY2 - Set VX to VX AND VY
    fn op_8xy2(&mut self, x: usize, y: usize) -> Pc {
        self.v[x] = self.v[y] & self.v[x];

        Pc::Next
    }

    // 8XY3 - Set VX to VX XOR VY
    fn op_8xy3(&mut self, x: usize, y: usize) -> Pc {
        self.v[x] = self.v[y] ^ self.v[x];

        Pc::Next
    }

    // 8XY4 - Add the value of register VY to register VX
    //        Set VF to 01 if a carry occurs
    //        Set VF to 00 if a carry does not occur
    fn op_8xy4(&mut self, x: usize, y: usize) -> Pc {
        let res = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = res.0;

        if res.1 {
            self.v[15] = 1;
        } else {
            self.v[15] = 0;
        }

        Pc::Next
    }

    // 8XY5 - Subtract the value of register VY from register VX
    //        Set VF to 00 if a borrow occurs
    //        Set VF to 01 if a borrow does not occur
    fn op_8xy5(&mut self, x: usize, y: usize) -> Pc {
        let res = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = res.0;

        if res.1 {
            self.v[15] = 0;
        } else {
            self.v[15] = 1;
        }

        Pc::Next
    }

    // 8XY6 - Store the value of register VY shifted right one bit in register VX
    //        Set register VF to the least significant bit prior to the shift
    //        VY is unchanged
    fn op_8xy6(&mut self, x: usize, y: usize) -> Pc {
        self.v[15] = self.v[y] & 1;
        self.v[x] = self.v[y] >> 1;

        Pc::Next
    }

    // 8XY7 - Set register VX to the value of VY minus VX
    //        Set VF to 00 if a borrow occurs
    //        Set VF to 01 if a borrow does not occur
    fn op_8xy7(&mut self, x: usize, y: usize) -> Pc {
        let res = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = res.0;

        if res.1 {
            self.v[15] = 0;
        } else {
            self.v[15] = 1;
        }

        Pc::Next
    }

    // 8XYE - Store the value of register VY shifted left one bit in register VX
    //        Set register VF to the most significant bit prior to the shift
    //        VY is unchanged
    fn op_8xye(&mut self, x: usize, y: usize) -> Pc {
        self.v[15] = (self.v[y] >> 7) & 1;
        self.v[x] = self.v[y] << 1;

        Pc::Next
    }

    // 9XY0 - Skip the following instruction if the value of register VX is not equal to the value of register VY
    fn op_9xy0(&self, x: usize, y: usize) -> Pc {
        if self.v[x] != self.v[y] {
            return Pc::Skip(1)
        }

        Pc::Next
    }

    // ANNN - Store memory address NNN in register I
    fn op_annn(&mut self, nnn: usize) -> Pc {
        self.i = nnn;

        Pc::Next
    }

    // BNNN - Jump to address NNN + V0
    fn op_bnnn (&self, nnn: usize) -> Pc {
        Pc::Jump(nnn + self.v[0] as usize)
    }

    // CXNN - Set VX to a random number with a mask of NN
    fn op_cxnn(&self, x: usize, nn: u8) -> Pc {
        unimplemented!()
    }

    // DXYN - Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
    //        Set VF to 01 if any set pixels are changed to unset, and 00 otherwise.
    //        Sprites should wrap horizontally and vertically
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) -> Pc {
        let vx = self.v[x] as usize;
        let vy = self.v[y] as usize;
        
        for byte in 0..n {
            let y = (vy + byte) % V_HEIGHT;
            
            for bit in 0..8usize {
                let x = (vx + bit) % V_WIDTH;
                
                let pixel = (self.memory[self.i + byte] >> (7 - bit)) & 1;
                self.v[15] = pixel & self.vram[y][x];
                self.vram[y][x] ^= pixel
            }
        }

        Pc::Next
    }

    // EX9E - Skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed
    fn op_ex9e(&self, x: usize) -> Pc {
        unimplemented!()
    }

    // EXA1 - Skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed
    fn op_exa1(&self, x: usize) -> Pc {
        unimplemented!()
    }

    // FX07 - Store the current value of the delay timer in register VX
    fn op_fx07(&self, x: usize) -> Pc {
        unimplemented!()
    }

    // FX0A - Wait for a keypress and store the result in register VX
    fn op_fx0a(&self, x: usize) -> Pc {
        unimplemented!()
    }

    // FX15	- Set the delay timer to the value of register VX
    fn op_fx15(&self, x: usize) -> Pc {
        unimplemented!()
    }
    
    // FX18	- Set the sound timer to the value of register VX
    fn op_fx18(&self, x: usize) -> Pc {
        unimplemented!()
    }

    // FX1E - Add the value stored in register VX to register I
    fn op_fx1e(&self, x: usize) -> Pc {
        unimplemented!()
    }

    // FX29	- Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
    fn op_fx29(&self, x: usize) -> Pc {
        unimplemented!()
    }

    // FX33	- Store the binary-coded decimal equivalent of the value stored in register VX at addresses I, I + 1, and I + 2
    fn op_fx33(&mut self, x: usize) -> Pc {
        self.memory[self.i] = self.v[x] / 100;
        self.memory[self.i + 1] = (self.v[x] % 100) / 10;
        self.memory[self.i + 2] = self.v[x] % 10;

        Pc::Next
    }

    // FX55	- Store the values of registers V0 to VX inclusive in memory starting at address I
    //        I is set to I + X + 1 after operation
    fn op_fx55(&mut self, x: usize) -> Pc {
        for pos in 0..=x {
            self.memory[self.i + pos] = self.v[pos];
        }

        self.i += x + 1;

        Pc::Next
    }

    // FX65	- Fill registers V0 to VX inclusive with the values stored in memory starting at address I
    //        I is set to I + X + 1 after operation
    fn op_fx65(&mut self, x: usize) -> Pc {
        for pos in 0..=x {
            self.v[pos] = self.memory[self.i + pos];
        }

        self.i += x + 1;

        Pc::Next
    }

    fn no_op(&self) -> Pc {
        Pc::Next
    }
}

fn to_nybbles(op: (u8, u8)) -> (u8, u8, u8, u8) {
    (
        (op.0 & 0xF0) >> 4,
        (op.0 & 0x0F),
        (op.1 & 0xF0) >> 4,
        (op.1 & 0x0F),
    )
}

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;