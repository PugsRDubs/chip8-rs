use std::fs;

use crate::chip8::opcodes::*;

mod opcodes;

pub struct Chip8 {
    memory: [u8; 4096],
    reg: [u8; 16],
    delay_t: u8,
    sound_t: u8,
    pc: u16,
    stack: [u16; 16],
    sp: u8, 
    pub gfx: [bool; 64*32],
    keys: [bool; 16],
    index: u16,
    opcode: u16,
}

const FONT_START_ADDRESS: u16 = 0x50;
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

impl Chip8 {
    pub fn new() -> Self {
        Self { 
            memory: [0; 4096],
            reg: [0; 16],
            delay_t: 0,
            sound_t: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            gfx: [false; 64*32],
            keys: [false; 16],
            index: 0,
            opcode: 0,
        }
    }
    pub fn init(&mut self) -> () {
        // load font
        for i in 0..80 {
            self.memory[(FONT_START_ADDRESS + i) as usize] = FONT[i as usize];
        }

        self.pc = 0x200;
    }
    pub fn cycle(&mut self) -> () {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16);

        self.pc += 2;

        get_opcode_fn(&self.opcode)(self);
    }
    pub fn advance_time(&mut self) -> () {
        if self.delay_t > 0 {
            self.delay_t -= 1;
        }
        if self.sound_t > 0 {
            self.sound_t -= 1;
        }
    }
    pub fn load_rom(&mut self, path: String) {
        let address: usize = 0x200;
        let bytes = fs::read(path).unwrap();
        for i in 0..bytes.len() {
            self.memory[address + i] = bytes[i];
        }
    }
    pub fn _print(&mut self) -> () {
        let mut out: String = String::new();
        for y in 0..32 {
            for x in 0..64 {
                if self.gfx[(y*64 + x) as usize] {
                    out += "█";
                }
                else {
                    out += " ";
                }
            }
            out += "\n";
        }
        print!("{out}");
    }
    pub fn set_key(&mut self, key: usize, pressed: bool)
    {
        self.keys[key] = pressed;
    }
}

fn get_opcode_fn(opcode: &u16) -> fn(chip8: &mut Chip8) -> () {
    match opcode_first(opcode) {
        0x0 => ops_0(opcode),
        0x1 => op_1nnn,
        0x2 => op_2nnn,
        0x3 => op_3xkk,
        0x4 => op_4xkk,
        0x5 => op_5xy0,
        0x6 => op_6xkk,
        0x7 => op_7xkk,
        0x8 => ops_8(opcode),
        0x9 => op_9xy0,
        0xA => op_annn,
        0xB => op_bnnn,
        0xC => op_cxkk,
        0xD => op_dxyn,
        0xE => ops_e(opcode),
        0xF => ops_f(opcode),
        _ => op_null
    }
}

fn ops_0(opcode: &u16) -> fn(chip8: &mut Chip8) -> () {
    match opcode_nnn(opcode) {
        0x0E0 => op_00e0,
        0x0EE => op_00ee,
        _ => op_null
    }
}

fn ops_8(opcode: &u16) -> fn(chip8: &mut Chip8) -> () {
    match opcode_n(opcode) {
        0x0 => op_8xy0,
        0x1 => op_8xy1,
        0x2 => op_8xy2,
        0x3 => op_8xy3,
        0x4 => op_8xy4,
        0x5 => op_8xy5,
        0x6 => op_8xy6,
        0x7 => op_8xy7,
        0xE => op_8xye,
        _ => op_null
    }
}

fn ops_e(opcode: &u16) -> fn(chip8: &mut Chip8) -> () {
    match opcode_kk(opcode) {
        0x9E => op_ex9e,
        0xA1 => op_exa1,
        _ => op_null
    }
}

fn ops_f(opcode: &u16) -> fn(chip8: &mut Chip8) -> () {
    match opcode_kk(opcode) {
        0x07 => op_fx07,
        0x0A => op_fx0a,
        0x15 => op_fx15,
        0x18 => op_fx18,
        0x1E => op_fx1e,
        0x29 => op_fx29,
        0x33 => op_fx33,
        0x55 => op_fx55,
        0x65 => op_fx65,
        _ => op_null
    }
}