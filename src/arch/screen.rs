// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{error::Error, fs::File, io::BufWriter, mem::size_of, path::Path};

use crate::{Signal, Signal13, Signal16, Unit};

/// 512x256 black-and-white screen which behaves like a `Ram8k`.
pub struct Screen {
    inp: Signal16,
    address: Signal13,
    load: Signal,
    out: Signal16,

    selected_row: usize,
    selected_col: usize,
    pixels: Vec<[u16; 32]>,

    width: u32,
    height: u32,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            inp: Default::default(),
            address: Default::default(),
            load: Default::default(),
            out: Default::default(),
            selected_row: 0,
            selected_col: 0,
            pixels: vec![[0; 32]; 256],
            width: 512,
            height: 256,
        }
    }
}

impl Screen {
    /// Carries out the simulation, taking as input:
    ///
    /// - `inp`: data input
    /// - `load`: whether to write `inp` into the RAM
    /// - `address`: which register to select
    ///
    /// It returns the value stored at `address`. Furthermore, when `load` is
    /// `HI`, `inp` is written at location `address`, and the loaded value will
    /// be emitted from the next time step onward.
    ///
    /// The pixel at row _r_ from the top and column _c_ from the left, is mapped
    /// onto the `c % 16` bit (from LSB to MSB) of the 16-bit word stored at
    /// address `r * 32 + c / 16`.
    pub fn sim(&mut self, inp: Signal16, load: Signal, address: Signal13) -> Signal16 {
        self.inp = inp;
        self.load = load;
        self.address = address;
        self.tick();
        self.out
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.pixels.as_ptr() as *const u8,
                self.pixels.len() * self.pixels[0].len() * size_of::<u16>(),
            )
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u16 {
        let lane = self.pixels[y][x / 16];
        let mask = 1 << (x as u16 % 16);
        let value = lane & mask;
        if value == 0 {
            0
        } else {
            1
        }
    }

    pub fn out(&self) -> Signal16 {
        self.out
    }

    pub fn set_address(&mut self, address: impl Into<Signal13>) {
        self.address = address.into();
    }

    pub fn dump(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let png_file = File::create(path)?;
        let buf_writer = BufWriter::new(png_file);
        let mut encoder = png::Encoder::new(buf_writer, self.width, self.height);

        encoder.set_depth(png::BitDepth::One);
        // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
        // 1.0 / 2.2, unscaled, but rounded
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
        // Using unscaled instantiation here
        let source_chromaticities = png::SourceChromaticities::new(
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(self.as_bytes())?;

        Ok(())
    }
}

impl Unit for Screen {
    fn tick(&mut self) {
        let address = usize::from(self.address);
        let row = address / 32;
        let col = address % 32;

        self.selected_row = row;
        self.selected_col = col;
        self.eval();

        if self.load.as_bool() {
            self.pixels[row][col] = self.inp.into();
        }
    }

    fn eval(&mut self) {
        self.out = self.pixels[self.selected_row][self.selected_col].into();
    }
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::create_dir, path::Path};

    use super::*;

    #[test]
    fn run() -> Result<(), Box<dyn Error>> {
        let screen_path = Path::new("target/screen");
        if !screen_path.exists() {
            create_dir(screen_path)?;
        }

        let mut screen = Screen::default();
        screen.dump(screen_path.join("black.png"))?;

        screen.pixels.fill([u16::MAX; 32]);
        screen.dump(screen_path.join("white.png"))?;

        Ok(())
    }
}
