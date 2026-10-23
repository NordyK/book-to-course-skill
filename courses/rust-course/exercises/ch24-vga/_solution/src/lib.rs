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
    (bg as u8) << 4 | fg as u8
}

pub fn cell(ch: u8, attr: u8) -> u16 {
    (attr as u16) << 8 | ch as u16
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
        assert_eq!(buf.len(), width * height, "buffer size must be width * height");
        let mut w = Writer { buf, width, height, row: 0, col: 0, attr: 0x07 };
        w.clear();
        w
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.attr = attribute(fg, bg);
    }

    pub fn position(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    fn blank(&self) -> u16 {
        cell(b' ', self.attr)
    }

    fn newline(&mut self) {
        self.col = 0;
        if self.row + 1 < self.height {
            self.row += 1;
        } else {
            let w = self.width;
            self.buf.copy_within(w.., 0);
            let blank = self.blank();
            let last = (self.height - 1) * w;
            self.buf[last..].fill(blank);
        }
    }

    pub fn write_byte(&mut self, b: u8) {
        let ch = match b {
            b'\n' => {
                self.newline();
                return;
            }
            0x20..=0x7e => b,
            _ => 0xfe,
        };
        if self.col == self.width {
            self.newline();
        }
        self.buf[self.row * self.width + self.col] = cell(ch, self.attr);
        self.col += 1;
    }

    pub fn clear(&mut self) {
        let blank = self.blank();
        self.buf.fill(blank);
        self.row = 0;
        self.col = 0;
    }
}

impl fmt::Write for Writer<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.write_byte(b);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
