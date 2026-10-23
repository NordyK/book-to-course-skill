pub fn clamp_percent(v: i32) -> u8 {
    todo!()
}

pub struct Temperature {
    kelvin: f64,
}

impl Temperature {
    pub fn from_kelvin(k: f64) -> Temperature {
        todo!()
    }

    pub fn celsius(&self) -> f64 {
        todo!()
    }
}

pub fn parse_kv(s: &str) -> Result<(String, i32), String> {
    todo!()
}

#[cfg(test)]
mod tests;
