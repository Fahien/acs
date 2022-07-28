// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

pub mod fast;

pub mod time;
pub use time::*;

pub mod dff;
pub use dff::*;
pub mod bit;
pub use bit::*;
pub mod register16;
pub use register16::*;
pub mod counter;
pub use counter::*;

pub mod ram8;
pub use ram8::*;
pub mod ram64;
pub use ram64::*;
pub mod ram512;
pub use ram512::*;
pub mod ram4k;
pub use ram4k::*;
pub mod ram16k;
pub use ram16k::*;
