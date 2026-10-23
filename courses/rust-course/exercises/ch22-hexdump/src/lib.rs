use std::io::{self, Read};

pub fn hexdump<R: Read>(mut input: R) -> io::Result<String> {
    todo!()
}

pub fn crc32(data: &[u8]) -> u32 {
    todo!()
}

pub fn encode_record(key: &[u8], value: &[u8]) -> Vec<u8> {
    todo!()
}

pub fn decode_record<R: Read>(r: &mut R) -> io::Result<(Vec<u8>, Vec<u8>)> {
    todo!()
}

#[cfg(test)]
mod tests;
