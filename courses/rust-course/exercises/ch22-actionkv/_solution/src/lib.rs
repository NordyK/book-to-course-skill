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

enum Next {
    Record(Vec<u8>, Vec<u8>),
    End,
}

fn read_record<R: Read>(f: &mut R) -> io::Result<Next> {
    let mut header = [0u8; 12];
    match f.read_exact(&mut header) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(Next::End),
        Err(e) => return Err(e),
    }
    let saved = u32::from_le_bytes(header[0..4].try_into().unwrap());
    let key_len = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
    let val_len = u32::from_le_bytes(header[8..12].try_into().unwrap()) as usize;
    let mut body = vec![0u8; key_len + val_len];
    match f.read_exact(&mut body) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(Next::End),
        Err(e) => return Err(e),
    }
    if crc32(&body) != saved {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "checksum mismatch"));
    }
    let value = body.split_off(key_len);
    Ok(Next::Record(body, value))
}

impl<F: Read + Write + Seek> ActionKV<F> {
    pub fn open(mut f: F) -> io::Result<Self> {
        let mut index = HashMap::new();
        f.seek(SeekFrom::Start(0))?;
        loop {
            let pos = f.stream_position()?;
            match read_record(&mut f)? {
                Next::Record(key, value) => {
                    if value.is_empty() {
                        index.remove(&key);
                    } else {
                        index.insert(key, pos);
                    }
                }
                Next::End => break,
            }
        }
        Ok(ActionKV { f, index })
    }

    pub fn get(&mut self, key: &[u8]) -> io::Result<Option<Vec<u8>>> {
        let Some(&pos) = self.index.get(key) else {
            return Ok(None);
        };
        self.f.seek(SeekFrom::Start(pos))?;
        match read_record(&mut self.f)? {
            Next::Record(_, v) => Ok(Some(v)),
            Next::End => Err(io::Error::new(io::ErrorKind::UnexpectedEof, "record vanished")),
        }
    }

    pub fn insert(&mut self, key: &[u8], value: &[u8]) -> io::Result<()> {
        let pos = self.f.seek(SeekFrom::End(0))?;
        let mut body = Vec::with_capacity(key.len() + value.len());
        body.extend_from_slice(key);
        body.extend_from_slice(value);
        let mut rec = Vec::with_capacity(12 + body.len());
        rec.extend_from_slice(&crc32(&body).to_le_bytes());
        rec.extend_from_slice(&(key.len() as u32).to_le_bytes());
        rec.extend_from_slice(&(value.len() as u32).to_le_bytes());
        rec.extend_from_slice(&body);
        self.f.write_all(&rec)?;
        self.f.flush()?;
        self.index.insert(key.to_vec(), pos);
        Ok(())
    }

    pub fn update(&mut self, key: &[u8], value: &[u8]) -> io::Result<()> {
        self.insert(key, value)
    }

    pub fn delete(&mut self, key: &[u8]) -> io::Result<()> {
        if !self.index.contains_key(key) {
            return Ok(());
        }
        self.insert(key, b"")?;
        self.index.remove(key);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn compact<G: Read + Write + Seek>(&mut self, into: G) -> io::Result<ActionKV<G>> {
        let mut keys: Vec<Vec<u8>> = self.index.keys().cloned().collect();
        keys.sort();
        let mut out = ActionKV::open(into)?;
        for key in keys {
            if let Some(v) = self.get(&key)? {
                out.insert(&key, &v)?;
            }
        }
        Ok(out)
    }

    pub fn into_inner(self) -> F {
        self.f
    }
}

#[cfg(test)]
mod tests;
