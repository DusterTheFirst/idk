use std::str::FromStr;

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

                sudoku[block][cell].digit = if char == '-' {
                    None
                } else {
                    Some(
                        char.to_digit(10)
                            .ok_or(SudokuParseError::InvalidChar(char))?
                            as u8,
                    )
                }
            }
        }

        Ok(sudoku)
    }
}

#[derive(Default, Debug, Deref, DerefMut, AsRef, Clone)]
pub struct Block([Cell; 9]);

#[derive(Default, Debug, Clone)]
pub struct Cell {
    pub digit: Option<u8>,
    pub pencil: Vec<u8>,
}
