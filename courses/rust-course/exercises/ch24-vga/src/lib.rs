#![cfg_attr(not(test), no_std)]

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0, Blue = 1, Green = 2, Cyan = 3,
    Red = 4, Magenta = 5, Brown = 6, Gray = 7,
    DarkGray = 8, BrightBlue = 9, BrightGreen = 10, BrightCyan = 11,
    BrightRed = 12, BrightMagenta = 13, Yellow = 14, White = 15,
}

pub fn attribute(fg: Color, bg: Color) -> u8 {
    todo!()
}

pub fn cell(ch: u8, attr: u8) -> u16 {
    todo!()
}

pub struct Writer<'a> {
    buf: &'a mut [u16],
    width: usize,
    height: usize,
    row: usize,
    col: usize,
    attr: u8,
}

impl<'a> Writer<'a> {
    pub fn new(buf: &'a mut [u16], width: usize, height: usize) -> Writer<'a> {
        todo!()
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        todo!()
    }

    pub fn position(&self) -> (usize, usize) {
        todo!()
    }

    pub fn write_byte(&mut self, b: u8) {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }
}

impl fmt::Write for Writer<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests;
