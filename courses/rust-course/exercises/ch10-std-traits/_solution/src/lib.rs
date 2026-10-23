use std::fmt;
use std::ops::{Add, AddAssign, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Money {
    pub cents: i64,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.cents < 0 { "-" } else { "" };
        let c = self.cents.abs();
        write!(f, "{sign}${}.{:02}", c / 100, c % 100)
    }
}

impl From<i64> for Money {
    fn from(dollars: i64) -> Self {
        Money { cents: dollars * 100 }
    }
}

impl TryFrom<&str> for Money {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let err = || format!("bad amount: {s}");
        let (negative, body) = match s.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, s),
        };
        let (dollars, cents) = match body.split_once('.') {
            Some((d, c)) => {
                if c.len() != 2 || !c.chars().all(|ch| ch.is_ascii_digit()) {
                    return Err(err());
                }
                (d, c.parse::<i64>().map_err(|_| err())?)
            }
            None => (body, 0),
        };
        if dollars.is_empty() || !dollars.chars().all(|ch| ch.is_ascii_digit()) {
            return Err(err());
        }
        let dollars: i64 = dollars.parse().map_err(|_| err())?;
        let total = dollars * 100 + cents;
        Ok(Money { cents: if negative { -total } else { total } })
    }
}

impl Add for Money {
    type Output = Money;
    fn add(self, o: Money) -> Money {
        Money { cents: self.cents + o.cents }
    }
}

impl Sub for Money {
    type Output = Money;
    fn sub(self, o: Money) -> Money {
        Money { cents: self.cents - o.cents }
    }
}

impl Neg for Money {
    type Output = Money;
    fn neg(self) -> Money {
        Money { cents: -self.cents }
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, o: Money) {
        self.cents += o.cents;
    }
}

impl std::iter::Sum for Money {
    fn sum<I: Iterator<Item = Money>>(iter: I) -> Money {
        iter.fold(Money::default(), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests;
