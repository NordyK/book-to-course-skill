pub fn clamp_percent(v: i32) -> u8 {
    v.clamp(0, 100) as u8
}

pub struct Temperature {
    kelvin: f64,
}

impl Temperature {
    pub fn from_kelvin(k: f64) -> Temperature {
        if k < 0.0 {
            panic!("temperature below absolute zero: {k}");
        }
        Temperature { kelvin: k }
    }

    pub fn celsius(&self) -> f64 {
        self.kelvin - 273.15
    }
}

pub fn parse_kv(s: &str) -> Result<(String, i32), String> {
    let (k, v) = s.split_once('=').ok_or(String::from("missing ="))?;
    let k = k.trim();
    if k.is_empty() {
        return Err(String::from("empty key"));
    }
    let v: i32 = v.trim().parse().map_err(|_| String::from("bad number"))?;
    Ok((k.to_string(), v))
}

#[cfg(test)]
mod tests;
