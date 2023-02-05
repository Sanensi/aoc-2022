use std::{collections::HashMap, fmt::Debug};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    pub fn get_northward_positions(&self) -> Vec<Position> {
        vec![
            Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x + 1,
                y: self.y - 1,
            },
        ]
    }

    pub fn get_southward_positions(&self) -> Vec<Position> {
        vec![
            Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }

    pub fn get_westward_positions(&self) -> Vec<Position> {
        vec![
            Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
            Position {
                x: self.x - 1,
                y: self.y + 1,
            },
        ]
    }

    pub fn get_eastward_positions(&self) -> Vec<Position> {
        vec![
            Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }

    pub fn get_adjacent_positions(&self) -> Vec<Position> {
        vec![
            Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct InfiniteGrid<T> {
    grid: HashMap<Position, T>,
}

impl<T> InfiniteGrid<T> {
    pub fn new<E>(elements: E) -> InfiniteGrid<T>
    where
        E: Iterator<Item = (Position, T)>,
    {
        InfiniteGrid {
            grid: elements.collect(),
        }
    }

    pub fn get(&self, p: &Position) -> Option<&T> {
        self.grid.get(p)
    }

    pub fn insert(&mut self, p: Position, elf: T) -> Option<T> {
        self.grid.insert(p, elf)
    }

    pub fn remove(&mut self, p: &Position) -> Option<T> {
        self.grid.remove(p)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Position, &T)> {
        self.grid.iter()
    }

    pub fn len(&self) -> usize {
        self.grid.len()
    }
}
