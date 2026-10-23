//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::{PrimaryColor, SecondaryColor};
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two different primary colors in equal amounts into a secondary color.
    ///
    /// Returns `None` when both colors are the same.
    ///
    /// # Examples
    ///
    /// ```
    /// use ch14_api::{mix, PrimaryColor, SecondaryColor};
    /// assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Blue), Some(SecondaryColor::Purple));
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Option<SecondaryColor> {
        use PrimaryColor::*;
        use SecondaryColor::*;
        match (c1, c2) {
            (Red, Yellow) | (Yellow, Red) => Some(Orange),
            (Yellow, Blue) | (Blue, Yellow) => Some(Green),
            (Red, Blue) | (Blue, Red) => Some(Purple),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests;
