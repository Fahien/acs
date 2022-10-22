// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

pub mod signal;
pub use signal::*;

pub mod unit;
pub use unit::*;

pub mod nand;
pub use nand::*;
pub mod not;
pub use not::*;
pub mod and;
pub use and::*;
pub mod or;
pub use or::*;
pub mod xor;
pub use xor::*;
pub mod mux;
pub use mux::*;
pub mod demux;
pub use demux::*;

pub mod signal16;
pub use signal16::*;

pub mod nand16;
pub use nand16::*;
pub mod not16;
pub use not16::*;
pub mod and16;
pub use and16::*;
pub mod or16;
pub use or16::*;
pub mod mux16;
pub use mux16::*;

pub mod signal2;
pub use signal2::*;
pub mod signal3;
pub use signal3::*;
pub mod signal6;
pub use signal6::*;
pub mod signal8;
pub use signal8::*;
pub mod signal9;
pub use signal9::*;

pub mod or8way;
pub use or8way::*;
pub mod mux4way16;
pub use mux4way16::*;
pub mod mux8way16;
pub use mux8way16::*;
pub mod demux4way;
pub use demux4way::*;
pub mod demux8way;
pub use demux8way::*;
pub mod or16way;
pub use or16way::*;

pub mod halfadder;
pub use halfadder::*;
pub mod fulladder;
pub use fulladder::*;
pub mod add16;
pub use add16::*;
pub mod inc16;
pub use inc16::*;
pub mod alu;
pub use alu::*;
