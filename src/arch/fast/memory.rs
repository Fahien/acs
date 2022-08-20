// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{mem::fast::Ram16k, Keyboard, Screen, Signal, Signal15, Signal16, Unit};

/// Fast package of three 16-bit chip parts: `Ram16k`, `Screen`, and `Keyboard`
#[derive(Default)]
pub struct Memory {
    pub inp: Signal16,
    pub load: Signal,
    pub address: Signal15,
    pub out: Signal16,

    pub selected: Signal16,

    pub ram: Ram16k,
    pub screen: Screen,
    pub keyboard: Keyboard,
}

impl Memory {
    /// Carries out the simulation, taking as input:
    ///
    /// - `inp`: data input
    /// - `load`: whether to write `inp` into the RAM
    /// - `address`: which register to select
    ///   - \[0, 16383\] results in accessing Ram16k
    ///   - \[16384, 24575\] results in accessing the Screen
    ///   - 24576 results in accessing the Keyboard
    ///
    /// It returns the value stored at `address. Furthermore, when `load` is
    /// `HI`, `inp` is written at location `address`, and the loaded value will
    /// be emitted from the next time step onward.
    pub fn sim(&mut self, inp: Signal16, load: Signal, address: Signal15) -> Signal16 {
        self.inp = inp;
        self.load = load;
        self.address = address;
        self.tick();
        self.out
    }

    pub fn out(&self) -> Signal16 {
        self.out
    }

    pub fn get_screen(&self) -> &Screen {
        &self.screen
    }

    pub fn get_keyboard_mut(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }
}

impl Unit for Memory {
    fn tick(&mut self) {
        // When not_ram is 1, we are going into devices address space
        let not_ram = Signal::from(self.address.get(14..15));
        let sel_ram = !not_ram;

        let ram_load = self.load & sel_ram;

        // When not_screen is 1 we are going beyond its address space
        let not_screen = Signal::from(self.address.get(13..14));
        let sel_screen = !not_screen;

        let screen_load = (not_ram & sel_screen) & self.load;

        let ram_out = self.ram.sim(self.inp, ram_load, self.address);
        let screen_out = self.screen.sim(self.inp, screen_load, self.address.into());
        self.keyboard.eval();

        // Only one address should select keyboard
        // Out is going to be 0 if address is keyboard
        // otherwise 1 if address is not keyboard
        let address_and_not_keyboard = Signal16::from(self.address) & !Signal16::new(24576);
        let not_keyboard_address = address_and_not_keyboard != Signal16::FALSE;
        let keyboard_out_out = if not_keyboard_address {
            Signal16::FALSE
        } else {
            self.keyboard.out()
        };

        // Select between screen and keyboard
        let screen_key_mux_out = if not_screen.as_bool() {
            keyboard_out_out
        } else {
            screen_out
        };

        // Select out between ram and devices
        let out_mux_out = if not_ram.as_bool() {
            screen_key_mux_out
        } else {
            ram_out
        };

        self.out = out_mux_out;
    }

    fn eval(&mut self) {
        // When not_ram is 1, we are going into devices address space
        let not_ram = Signal::from(self.address.get(14..15));
        let sel_ram = !not_ram;

        let ram_load = self.load & sel_ram;

        // When not_screen is 1 we are going beyond its address space
        let not_screen = Signal::from(self.address.get(13..14));
        let sel_screen = !not_screen;

        let screen_load = (not_ram & sel_screen) & self.load;

        self.ram.set_inp(self.inp);
        self.ram.set_address(self.address);
        self.ram.set_load(ram_load);
        self.ram.eval();
        let ram_out = self.ram.out();
        let screen_out = self.screen.sim(self.inp, screen_load, self.address.into());
        self.screen.eval();
        self.keyboard.eval();

        // Only one address should select keyboard
        // Out is going to be 0 is address is keyboard
        // otherwise 1 if address is not keyboard
        let address_and_not_keyboard = Signal16::from(self.address) & !Signal16::new(24576);
        let not_keyboard_address = address_and_not_keyboard != Signal16::FALSE;
        let keyboard_out_out = if not_keyboard_address {
            Signal16::FALSE
        } else {
            self.keyboard.out()
        };

        // Select between screen and keyboard
        let screen_key_mux_out = if not_screen.as_bool() {
            keyboard_out_out
        } else {
            screen_out
        };

        // Select out between ram and devices
        let out_mux_out = if not_ram.as_bool() {
            screen_key_mux_out
        } else {
            ram_out
        };

        self.out = out_mux_out;
    }
}
