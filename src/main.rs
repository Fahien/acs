use std::{
    env,
    error::Error,
    fs::{read_to_string, File},
    io::{self, BufRead},
    path::Path,
    time::{Duration, Instant},
};

use acs::{Assembler, Computer, Unit};
use sdl::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};
use sdl2 as sdl;

// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let asm_path = args.get(1).expect("Expected one cli argument: asm_path");
    let mut computer = Computer::default();

    let code = read_to_string(asm_path).expect("Failed to read string from asm");
    let mut assembler = Assembler::default();
    let instructions = assembler.assemble(code);
    computer.set_instructions(instructions);

    let sdl = sdl::init()?;

    let video = sdl.video()?;

    let screen = computer.get_screen();
    let width = screen.get_width();
    let height = screen.get_height();

    let window = video
        .window("acs", width, height)
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut texture =
        texture_creator.create_texture_streaming(PixelFormatEnum::RGB332, width, height)?;

    canvas.clear();
    canvas.copy(&texture, None, None)?;
    canvas.present();

    let mut ips = 0;
    let mut ips_den = 1;

    let mut event_pump = sdl.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    computer.get_keyboard_mut().set(keycode);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    computer.get_keyboard_mut().unset(keycode);
                }
                _ => {}
            }
        }

        let start = Instant::now();

        let mut cicle_count = 0;
        const GROUP_MAX: usize = 64;
        while Instant::now() - start < Duration::from_millis(16) {
            for _ in 0..GROUP_MAX {
                computer.tick();
                computer.tock();
            }
            cicle_count += 1;
        }
        ips = (ips + cicle_count * GROUP_MAX * 60) / ips_den;
        ips_den = 2;

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..height {
                for x in 0..width {
                    let offset = y as usize * pitch + x as usize;
                    let pixel = computer.get_screen().get_pixel(x as usize, y as usize) as u8 * 255;
                    buffer[offset] = pixel;
                }
            }
        })?;

        canvas.copy(&texture, None, None)?;
        canvas.present();
    }

    println!("MIPS: {}", (ips as f32) / 1_000_000.0);

    Ok(())
}
