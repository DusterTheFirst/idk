use std::{
    collections::{BTreeSet, HashSet},
    convert::{TryFrom, TryInto},
    str::FromStr,
};

use derive_more::{AsRef, Deref, DerefMut};
use petgraph::{
    dot::{Config, Dot},
    graph::UnGraph,
    Graph,
};
use thiserror::Error;

#[derive(Default, Debug, Deref, DerefMut, AsRef, Clone)]
pub struct Sudoku([Block; 9]);

impl Sudoku {
    pub fn new() {
        let mut graph = Graph::new_undirected();
        let mut all_cells = Vec::new();

        for block in 0..9 {
            let x = block % 3 * 3;
            let y = block / 3 * 3;

            let mut cells = (0..9)
                .map(|cell| {
                    (
                        (x + cell % 3, y + cell / 3),
                        block,
                        graph.add_node(Cell::Pencil(BTreeSet::new())),
                    )
                })
                .collect::<Vec<_>>();

            // for (_, _, cell) in cells.iter() {
            //     for (_, _, other_cell) in cells.iter().filter(|(_, _, idx)| idx != cell) {
            //         graph.add_edge(*cell, *other_cell, ());
            //     }
            // }

            all_cells.append(&mut cells);
        }

        for ((x, y), block, cell) in all_cells.iter() {
            for ((other_x, other_y), other_block, other_cell) in all_cells.iter().filter(|(_, _, idx)| idx != cell) {
                if other_x == x || other_y == y || other_block == block {
                    graph.add_edge(*cell, *other_cell, ());
                }
            }
        }

        // for ((x, y), block, cell) in all_cells.iter() {
        //     for row_cell in all_cells
        //         .iter()
        //         .filter(|((other_x, _), other_block, id)| {
        //             other_x == x && id != cell && other_block != block
        //         })
        //         .map(|(_, _, idx)| idx)
        //     {
        //         graph.add_edge(*cell, *row_cell, ());
        //     }

        //     for col_cell in all_cells
        //         .iter()
        //         .filter(|((_, other_y), other_block, id)| {
        //             other_y == y && id != cell && other_block != block
        //         })
        //         .map(|(_, _, idx)| idx)
        //     {
        //         graph.add_edge(*cell, *col_cell, ());
        //     }
        // }

        eprintln!("done");
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    }

    pub fn solved(&self) -> bool {
        // TODO: Check rows and columns
        self.iter().all(|x| x.valid())
    }
}

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
                    sudoku[block][cell] = Cell::Given(
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

impl Block {
    pub fn valid(&self) -> bool {
        let mut unique = HashSet::new();
        self.iter().all(move |x| match x {
            Cell::Digit(digit) | Cell::Given(digit) => unique.insert(digit),
            _ => true,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    Digit(Digit),
    Given(Digit),
    Pencil(BTreeSet<Digit>),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Pencil(BTreeSet::new())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Digit {
    pub fn iterator() -> impl Iterator<Item = Digit> {
        [
            Digit::One,
            Digit::Two,
            Digit::Three,
            Digit::Four,
            Digit::Five,
            Digit::Six,
            Digit::Seven,
            Digit::Eight,
            Digit::Nine,
        ]
        .iter()
        .copied()
    }
}
