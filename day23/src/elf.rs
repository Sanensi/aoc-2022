use std::fmt::Display;

use crate::infinite_grid::{InfiniteGrid, Position};

static mut ELF_COUNT: usize = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elf {
    id: usize,
}

impl Elf {
    pub fn new() -> Elf {
        unsafe {
            let elf = Elf { id: ELF_COUNT };
            ELF_COUNT += 1;
            elf
        }
    }
}

impl Display for InfiniteGrid<Elf> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for (Position { x, y }, _) in self.iter() {
            if *x < min_x {
                min_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }

        let mut lines = vec![];
        for y in min_y..=max_y {
            let mut line = vec![];
            for x in min_x..=max_x {
                if self.get(&Position { x, y }).is_some() {
                    line.push('#');
                } else {
                    line.push('.')
                }
            }
            lines.push(line.iter().collect::<String>())
        }

        write!(f, "{}", lines.join("\n"))
    }
}
