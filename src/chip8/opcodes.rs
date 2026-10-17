use crate::chip8::{Chip8, FONT_START_ADDRESS};

pub fn opcode_first(opcode: &u16) -> u8 {
    ((opcode & 0xF000) >> 12) as u8
}

pub fn opcode_n(opcode: &u16) -> u8 {
    (opcode & 0x000F) as u8
}

pub fn opcode_nnn(opcode: &u16) -> u16 {
    opcode & 0x0FFF
}

pub fn opcode_kk(opcode: &u16) -> u8 {
    (opcode & 0x00FF) as u8
}

pub fn opcode_x(opcode: &u16) -> u8 {
    ((opcode & 0x0F00) >> 8) as u8
}

pub fn opcode_y(opcode: &u16) -> u8 {
    ((opcode & 0x00F0) >> 4) as u8
}

pub fn op_null(_: &mut Chip8) -> () {}

pub fn op_00e0(c: &mut Chip8) -> () {
    c.gfx = [false; 64*32];
}

pub fn op_00ee(c: &mut Chip8) -> () {
    c.sp-=1;
    c.pc = c.stack[c.sp as usize];
}

pub fn op_1nnn(c: &mut Chip8) -> () {
    c.pc = opcode_nnn(&c.opcode)
}

pub fn op_2nnn(c: &mut Chip8) -> () {
    c.stack[c.sp as usize] = c.pc;
    c.sp += 1;
    c.pc = opcode_nnn(&c.opcode);
}

pub fn op_3xkk(c: &mut Chip8) -> () {
    if c.reg[opcode_x(&c.opcode) as usize] == opcode_kk(&c.opcode) {
        c.pc += 2;
    }
}

pub fn op_4xkk(c: &mut Chip8) -> () {
    if c.reg[opcode_x(&c.opcode) as usize] != opcode_kk(&c.opcode) {
        c.pc += 2;
    }
}

pub fn op_5xy0(c: &mut Chip8) -> () {
    if c.reg[opcode_x(&c.opcode) as usize] == c.reg[opcode_y(&c.opcode) as usize] {
        c.pc += 2;
    }
}

pub fn op_6xkk(c: &mut Chip8) -> () {
    c.reg[opcode_x(&c.opcode) as usize] = opcode_kk(&c.opcode);
}

pub fn op_7xkk(c: &mut Chip8) -> () {
    let (sum, _) = c.reg[opcode_x(&c.opcode) as usize].overflowing_add(opcode_kk(&c.opcode));
    c.reg[opcode_x(&c.opcode) as usize] = sum;
}

pub fn op_8xy0(c: &mut Chip8) -> () {
    c.reg[opcode_x(&c.opcode) as usize] = c.reg[opcode_y(&c.opcode) as usize];
}

pub fn op_8xy1(c: &mut Chip8) -> () {
    c.reg[opcode_x(&c.opcode) as usize] |= c.reg[opcode_y(&c.opcode) as usize];
}

pub fn op_8xy2(c: &mut Chip8) -> () {
    c.reg[opcode_x(&c.opcode) as usize] &= c.reg[opcode_y(&c.opcode) as usize];
}

pub fn op_8xy3(c: &mut Chip8) -> () {
    c.reg[opcode_x(&c.opcode) as usize] ^= c.reg[opcode_y(&c.opcode) as usize];
}

pub fn op_8xy4(c: &mut Chip8) -> () {
    let (sum, overflow) = c.reg[opcode_x(&c.opcode) as usize].overflowing_add(c.reg[opcode_y(&c.opcode) as usize]);
    c.reg[0xF] = overflow as u8;
    c.reg[opcode_x(&c.opcode) as usize] = sum;
}

pub fn op_8xy5(c: &mut Chip8) -> () {
    let (diff, overflow) = c.reg[opcode_x(&c.opcode) as usize].overflowing_sub(c.reg[opcode_y(&c.opcode) as usize]);
    c.reg[0xF] = overflow as u8;
    c.reg[opcode_x(&c.opcode) as usize] = diff;
}

pub fn op_8xy6(c: &mut Chip8) -> () {
    c.reg[0xF] = (c.reg[opcode_x(&c.opcode) as usize] & 0x1 == 1) as u8;
    c.reg[opcode_x(&c.opcode) as usize] /= 2;
}

pub fn op_8xy7(c: &mut Chip8) -> () {
    c.reg[0xF] = (c.reg[opcode_y(&c.opcode) as usize] > c.reg[opcode_x(&c.opcode) as usize]) as u8;
    c.reg[opcode_x(&c.opcode) as usize] = c.reg[opcode_y(&c.opcode) as usize] - c.reg[opcode_x(&c.opcode) as usize];
}

pub fn op_8xye(c: &mut Chip8) -> () {
    let (product, overflow) = c.reg[opcode_x(&c.opcode) as usize].overflowing_mul(2);
    c.reg[0xF] = overflow as u8;
    c.reg[opcode_x(&c.opcode) as usize] = product;
}

pub fn op_9xy0(c: &mut Chip8) -> () {
    if c.reg[opcode_x(&c.opcode) as usize] != c.reg[opcode_y(&c.opcode) as usize] {
        c.pc += 2;
    }
}

pub fn op_annn(c: &mut Chip8) -> () {
    c.index = opcode_nnn(&c.opcode)
}

pub fn op_bnnn(c: &mut Chip8) -> () {
    c.pc = opcode_nnn(&c.opcode) + c.reg[0] as u16;
}

pub fn op_cxkk(c: &mut Chip8) -> () {
    c.reg[opcode_x(&c.opcode) as usize] = rand::random::<u8>() & opcode_kk(&c.opcode);
}

pub fn op_dxyn(c: &mut Chip8) -> () {
    //print!("hey");
    let x_pos: usize = (c.reg[opcode_x(&c.opcode) as usize] % 64) as usize;
    let y_pos: usize = (c.reg[opcode_y(&c.opcode) as usize] % 32) as usize;
    c.reg[0xF] = 0;
    for row in 0..opcode_n(&c.opcode) as usize {
        let sprite_byte: u8 = c.memory[(c.index + row as u16) as usize];
        //print!("{:b}", sprite_byte);
        for col in 0..8 {
            let sprite_pixel: bool = (sprite_byte & (0x80 >> col)) > 0;
            //print!(" {sprite_pixel}");
            let screen_pixel_index: usize = ((y_pos + row) * 64 + (x_pos + col)) as usize;

            if sprite_pixel {
                if c.gfx[screen_pixel_index] {
                    c.reg[0xF] = 1;
                }
                c.gfx[screen_pixel_index] ^= true;
            }
        }
    }
}

pub fn op_ex9e(c: &mut Chip8) -> () {
    if c.keys[c.reg[(opcode_x(&c.opcode)) as usize] as usize] {
        c.pc += 2;
    }
}

pub fn op_exa1(c: &mut Chip8) -> () {
    if !c.keys[c.reg[(opcode_x(&c.opcode)) as usize] as usize] {
        c.pc += 2;
    }
}

pub fn op_fx07(c: &mut Chip8) -> () {
    c.reg[(opcode_x(&c.opcode)) as usize] = c.delay_t;
}

pub fn op_fx0a(c: &mut Chip8) -> () {
    for i in 0..16 as u8 {
        if c.keys[i as usize] {
            c.reg[(opcode_x(&c.opcode)) as usize] = i;
            return;
        }
    }
    c.pc -= 2;
}

pub fn op_fx15(c: &mut Chip8) -> () {
    c.delay_t = c.reg[(opcode_x(&c.opcode)) as usize];
}

pub fn op_fx18(c: &mut Chip8) -> () {
    c.sound_t = c.reg[(opcode_x(&c.opcode)) as usize];
}

pub fn op_fx1e(c: &mut Chip8) -> () {
    c.index += c.reg[(opcode_x(&c.opcode)) as usize] as u16;
}

pub fn op_fx29(c: &mut Chip8) -> () {
    c.index = FONT_START_ADDRESS + 5 * c.reg[(opcode_x(&c.opcode)) as usize] as u16;
}

pub fn op_fx33(c: &mut Chip8) -> () {
    let mut value: u8 = c.reg[(opcode_x(&c.opcode)) as usize];

    c.memory[(c.index + 2) as usize] = value % 10;
    value /= 10;

    c.memory[(c.index + 1) as usize] = value % 10;
    value /= 10;

    c.memory[c.index as usize] = value % 10;
}

pub fn op_fx55(c: &mut Chip8) -> () {
    for i in 0..(opcode_x(&c.opcode) + 1) {
        c.memory[(c.index + i as u16) as usize] = c.reg[i as usize];
    }
}

pub fn op_fx65(c: &mut Chip8) -> () {
    for i in 0..(opcode_x(&c.opcode) + 1) {
        c.reg[i as usize] = c.memory[(c.index + i as u16) as usize];
    }
}
