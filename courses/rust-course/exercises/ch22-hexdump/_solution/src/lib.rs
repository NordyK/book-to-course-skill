use std::io::{self, Read};

pub fn hexdump<R: Read>(mut input: R) -> io::Result<String> {
    let mut data = Vec::new();
    input.read_to_end(&mut data)?;
    let mut out = String::new();
    for (i, chunk) in data.chunks(16).enumerate() {
        out.push_str(&format!("[0x{:08x}] ", i * 16));
        for b in chunk {
            out.push_str(&format!("{b:02x} "));
        }
        out.push_str(&"   ".repeat(16 - chunk.len()));
        out.push_str(" |");
        for &b in chunk {
            out.push(if (0x20..=0x7e).contains(&b) { b as char } else { '.' });
        }
        out.push_str("|\n");
    }
    Ok(out)
}

pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

pub fn encode_record(key: &[u8], value: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(key.len() + value.len());
    body.extend_from_slice(key);
    body.extend_from_slice(value);
    let mut rec = Vec::with_capacity(12 + body.len());
    rec.extend_from_slice(&crc32(&body).to_le_bytes());
    rec.extend_from_slice(&(key.len() as u32).to_le_bytes());
    rec.extend_from_slice(&(value.len() as u32).to_le_bytes());
    rec.extend_from_slice(&body);
    rec
}

pub fn decode_record<R: Read>(r: &mut R) -> io::Result<(Vec<u8>, Vec<u8>)> {
    let mut header = [0u8; 12];
    r.read_exact(&mut header)?;
    let saved = u32::from_le_bytes(header[0..4].try_into().unwrap());
    let key_len = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
    let val_len = u32::from_le_bytes(header[8..12].try_into().unwrap()) as usize;
    let mut body = vec![0u8; key_len + val_len];
    r.read_exact(&mut body)?;
    if crc32(&body) != saved {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "checksum mismatch"));
    }
    let value = body.split_off(key_len);
    Ok((body, value))
}

#[cfg(test)]
mod tests;
