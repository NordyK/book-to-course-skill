use std::collections::HashMap;
use std::io::{self, Read, Seek, SeekFrom, Write};

pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xEDB8_8320 & (crc & 1).wrapping_neg());
        }
    }
    !crc
}

pub struct ActionKV<F: Read + Write + Seek> {
    f: F,
    pub index: HashMap<Vec<u8>, u64>,
}

impl<F: Read + Write + Seek> ActionKV<F> {
    pub fn open(f: F) -> io::Result<Self> {
        todo!()
    }

    pub fn get(&mut self, key: &[u8]) -> io::Result<Option<Vec<u8>>> {
        todo!()
    }

    pub fn insert(&mut self, key: &[u8], value: &[u8]) -> io::Result<()> {
        todo!()
    }

    pub fn update(&mut self, key: &[u8], value: &[u8]) -> io::Result<()> {
        self.insert(key, value)
    }

    pub fn delete(&mut self, key: &[u8]) -> io::Result<()> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn compact<G: Read + Write + Seek>(&mut self, into: G) -> io::Result<ActionKV<G>> {
        todo!()
    }

    pub fn into_inner(self) -> F {
        self.f
    }
}

#[cfg(test)]
mod tests;
