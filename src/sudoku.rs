use std::{
    collections::{BTreeSet, HashSet},
    convert::{TryFrom, TryInto},
    ops::{Index, IndexMut},
    str::FromStr,
};

use derive_more::{AsRef, Deref, DerefMut};
use petgraph::{graph::NodeIndex, visit::EdgeRef, Graph, Undirected};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Relation {
    Block,
    Row,
    Column,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SolveStatus {
    Unsolved,
    Solved,
    Invalid,
}

#[derive(Debug, Deref, DerefMut, AsRef)]
pub struct Sudoku(Graph<Cell, Relation, Undirected>);

impl Sudoku {
    pub fn new() -> Self {
        let mut graph = Graph::new_undirected();
        let mut all_cells = Vec::with_capacity(9 * 9);

        for x in 0..9 {
            for y in 0..9 {
                all_cells.push((
                    (x, y),
                    x / 3 + y * 3,
                    graph.add_node(Cell::Pencil(BTreeSet::new())),
                ))
            }
        }

        for ((x, y), block, cell) in all_cells.iter() {
            for ((other_x, other_y), other_block, other_cell) in
                all_cells.iter().filter(|(_, _, idx)| idx != cell)
            {
                if other_block == block {
                    graph.update_edge(*cell, *other_cell, Relation::Block);
                } else if other_x == x {
                    graph.update_edge(*cell, *other_cell, Relation::Column);
                } else if other_y == y {
                    graph.update_edge(*cell, *other_cell, Relation::Row);
                }
            }
        }

        Sudoku(graph)
    }

    pub fn solved(&self) -> bool {
        // TODO: Check rows and columns
        // self.iter().all(|x| x.valid())
        false
    }

    pub fn block_status(&self, (block_x, block_y): (usize, usize)) -> SolveStatus {
        let mut digits = HashSet::new();

        for cell in self.neighbors((block_x * 3, block_y * 3), Relation::Block) {
            match cell {
                Cell::Digit(digit) | Cell::Given(digit) => {
                    if !digits.insert(digit) {
                        return SolveStatus::Invalid;
                    }
                }
                Cell::Pencil(_) => {
                    return SolveStatus::Unsolved;
                }
            }
        }

        SolveStatus::Solved
    }

    pub fn neighbors(
        &self,
        (x, y): (usize, usize),
        relation: Relation,
    ) -> impl Iterator<Item = &Cell> {
        self.0
            .edges(Self::index_of((x, y)))
            .filter(move |x| x.weight() == &relation)
            .map(move |x| &self.0[x.target()])
    }

    pub fn all_neighbors(&self, (x, y): (usize, usize)) -> impl Iterator<Item = &Cell> {
        self.0
            .edges(Self::index_of((x, y)))
            .map(move |x| &self.0[x.target()])
    }

    fn index_of((x, y): (usize, usize)) -> NodeIndex {
        NodeIndex::new(x * 9 + y)
    }
}

impl Index<(usize, usize)> for Sudoku {
    type Output = Cell;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        &self.0[Self::index_of(pos)]
    }
}

impl IndexMut<(usize, usize)> for Sudoku {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        &mut self.0[Self::index_of(pos)]
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

        let mut sudoku = Sudoku::new();

        for block in 0..9 {
            for cell in 0..9 {
                let x = (block % 3) * 3 + cell % 3;
                let y = (block / 3) * 3 + cell / 3;

                let char = s.chars().nth(x + y * 9).unwrap();

                if char != '-' {
                    sudoku[(x, y)] = Cell::Given(
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
