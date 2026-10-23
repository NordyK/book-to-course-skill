//! # Art
//!
//! A library for modeling artistic concepts.

// TODO: re-export PrimaryColor, SecondaryColor and mix here

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

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Option<SecondaryColor> {
        todo!()
    }
}

#[cfg(test)]
mod tests;
