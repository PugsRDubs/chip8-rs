extern crate sdl2;

use std::env;
use std::{thread, time::Duration};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

mod chip8;

const KEY_INPUTS: [Keycode; 16] = [Keycode::X, Keycode::Num1, Keycode::Num2, Keycode::Num3, Keycode::Q, Keycode::W, Keycode::E, Keycode::A, Keycode::S, Keycode::D, Keycode::Z, Keycode::C, Keycode::Num4, Keycode::R, Keycode::F, Keycode::V];

fn main() {
    let help_text = "please provide: rom, instructions per step, hz";
    let rom: String;
    let instructions_per_step: u32;
    let target_hz: u32;

    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("{help_text}");
        return;
    }
    rom = args[1].clone();
    instructions_per_step = match args[2].parse::<u32>() {
        Err(_) => {println!("{help_text}"); return;},
        Ok(value) => value
    };
    target_hz = match args[3].parse::<u32>() {
        Err(_) => {println!("{help_text}"); return;},
        Ok(value) => value
    };

    let mut chip8 = chip8::Chip8::new();
    chip8.init();
    chip8.load_rom(rom);
    
    // start starter code
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window( "chip8", 1280, 640)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_logical_size(64, 32).unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(code), .. } => {
                    for i in 0..16 {
                        if code == KEY_INPUTS[i] {
                            chip8.set_key(i, true);
                            break;
                        }
                    }
                },
                Event::KeyUp { keycode: Some(code), .. } => {
                    for i in 0..16 {
                        if code == KEY_INPUTS[i] {
                            chip8.set_key(i, false);
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        for _ in 0..instructions_per_step {
            chip8.cycle();
        }
        chip8.advance_time();
        //chip8.print();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        draw_screen(&chip8.gfx, &mut canvas);

        //thread::sleep(Duration::from_millis(17));

        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / target_hz));
    }
}

// TODO - rather than draw the whole screen each time, keep track of which pixels change each frame and draw only those?
fn draw_screen(gfx: &[bool; 64*32], canvas: &mut Canvas<Window>) {
    for y in 0..32 as i32 {
        for x in 0..64 as i32 {
            if gfx[(y*64+x) as usize] {
                match canvas.draw_point(Point::from((x, y))) {
                    Err(e) => eprint!("error printing point {x}, {y}: {e}"),
                    Ok(_) => ()
                }
            }
        }
    }
}
