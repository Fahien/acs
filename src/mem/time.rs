use std::{fmt::Display, ops::AddAssign};

/// Helper class for keeping track of time, advancing every `tick` and `tock`.
#[derive(Default)]
pub struct Time {
    time: usize,
}

impl From<i32> for Time {
    fn from(time: i32) -> Self {
        Self {
            time: time as usize,
        }
    }
}

impl AddAssign<usize> for Time {
    fn add_assign(&mut self, rhs: usize) {
        self.time += rhs;
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = if self.time % 2 == 0 { " " } else { "+" };
        let string = format!("{}{}", self.time / 2, suffix);
        Display::fmt(&string, f)
    }
}
