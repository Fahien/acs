// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    mem::fast::Ram16k, And, And16, Keyboard, Mux16, Not, Or16Way, Screen, Signal, Signal15,
    Signal16, Unit,
};

/// Package of three 16-bit chip parts: `Ram16k`, `Screen`, and `Keyboard`
#[derive(Default)]
pub struct Memory {
    pub inp: Signal16,
    pub load: Signal,
    pub address: Signal15,
    out: Signal16,

    ram: Ram16k,
    screen: Screen,
    keyboard: Keyboard,

    sel_ram: Not,
    load_ram: And,

    sel_screen: Not,
    load_screen0: And,
    load_screen1: And,

    keyboard_address_and: And16,
    keyboard_address_or: Or16Way,
    keyboard_out: Mux16,

    screen_key_mux: Mux16,
    out_mux: Mux16,
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
}

impl Unit for Memory {
    fn tick(&mut self) {
        // When not_ram is 1, we are going into devices address space
        let not_ram = self.address.get(14..15).into();

        let ram_load = self.load_ram.sim(self.load, self.sel_ram.sim(not_ram));

        // When not_screen is 1 we are going beyond its address space
        let not_screen = self.address.get(13..14).into();

        let screen_load = self.load_screen1.sim(
            self.load_screen0
                .sim(not_ram, self.sel_screen.sim(not_screen)),
            self.load,
        );

        let ram_out = self.ram.sim(self.inp, ram_load, self.address);
        let screen_out = self.screen.sim(self.inp, screen_load, self.address.into());
        self.keyboard.eval();

        // Only one address should select keyboard
        // Out is going to be 0 is address is keyboard
        // otherwise 1 if address is not keyboard
        let kao = self.keyboard_address_or.sim(
            self.keyboard_address_and
                .sim(self.address.into(), !Signal16::new(24576)),
        );

        let keyboard_out_out = self
            .keyboard_out
            .sim(self.keyboard.out(), Signal16::FALSE, kao);

        // Select between screen and keyboard
        let screen_key_mux_out = self
            .screen_key_mux
            .sim(screen_out, keyboard_out_out, not_screen);

        // Select out between ram and devices
        let out_mux_out = self.out_mux.sim(ram_out, screen_key_mux_out, not_ram);

        self.out = out_mux_out;
    }

    fn eval(&mut self) {
        // When not_ram is 1, we are going into devices address space
        let not_ram = self.address.get(14..15).into();

        let ram_load = self.load_ram.sim(self.load, self.sel_ram.sim(not_ram));

        // When not_screen is 1 we are going beyond its address space
        let not_screen = self.address.get(13..14).into();

        let screen_load = self.load_screen1.sim(
            self.load_screen0
                .sim(not_ram, self.sel_screen.sim(not_screen)),
            self.load,
        );

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
        let kao = self.keyboard_address_or.sim(
            self.keyboard_address_and
                .sim(self.address.into(), !Signal16::new(24576)),
        );

        let keyboard_out_out = self
            .keyboard_out
            .sim(self.keyboard.out(), Signal16::FALSE, kao);

        // Select between screen and keyboard
        let screen_key_mux_out = self
            .screen_key_mux
            .sim(screen_out, keyboard_out_out, not_screen);

        // Select out between ram and devices
        let out_mux_out = self.out_mux.sim(ram_out, screen_key_mux_out, not_ram);

        self.out = out_mux_out;
    }
}
