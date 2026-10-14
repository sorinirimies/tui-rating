#![allow(dead_code)]
use arbitrary::Arbitrary;
use ratatui::layout::{Alignment, Rect};

#[derive(Arbitrary, Debug)]
pub struct Area {
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
}

impl Area {
    /// Bounded rect (<=200x100), possibly zero-size and offset from origin.
    pub fn rect(&self) -> Rect {
        Rect::new(self.x as u16, self.y as u16, (self.w as u16) % 201, (self.h as u16) % 101)
    }
}

pub fn align(n: u8) -> Alignment {
    match n % 3 {
        0 => Alignment::Left,
        1 => Alignment::Center,
        _ => Alignment::Right,
    }
}

/// Max item count kept bounded so fuzzing doesn't just exhaust memory.
pub fn cap(max: u32) -> usize {
    (max % 2048) as usize
}
