use std::{
    collections::BTreeSet,
    convert::{TryFrom, TryInto},
    str::FromStr,
};

use derive_more::{AsRef, Deref, DerefMut};
use thiserror::Error;

#[derive(Default, Debug, Deref, DerefMut, AsRef, Clone)]
pub struct Sudoku([Block; 9]);

#[derive(Error, Debug, Clone, Copy)]
pub enum SudokuParseError {
    #[error("the sudoku board was of invalid size. expected length of 81, found {0}")]
    TooShort(usize),
    #[error("encountered an invalid character: {0}")]
    InvalidChar(char),
}

impl FromStr for Sudoku {
    type Err = SudokuParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("\n", "");

        if s.len() != 81 {
            return Err(SudokuParseError::TooShort(s.len()));
        }

        let mut sudoku = Sudoku::default();

        for block in 0..9 {
            for cell in 0..9 {
                let x = (block % 3) * 3 + cell % 3;
                let y = (block / 3) * 3 + cell / 3;

                let char = s.chars().nth(x + y * 9).unwrap();

                if char != '-' {
                    sudoku[block][cell] = Cell::Digit(
                        (char
                            .to_digit(10)
                            .ok_or(SudokuParseError::InvalidChar(char))?
                            as u8)
                            .try_into()
                            .map_err(|_| SudokuParseError::InvalidChar(char))?,
                    )
                }
            }
        }

        Ok(sudoku)
    }
}

#[derive(Default, Debug, Deref, DerefMut, AsRef, Clone)]
pub struct Block([Cell; 9]);

#[derive(Debug, Clone)]
pub enum Cell {
    Digit(Digit),
    Pencil(BTreeSet<Digit>),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Pencil(BTreeSet::new())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Digit {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl TryFrom<u8> for Digit {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            _ => Err(()),
        }
    }
}

impl From<Digit> for u8 {
    fn from(digit: Digit) -> Self {
        digit as u8
    }
}
