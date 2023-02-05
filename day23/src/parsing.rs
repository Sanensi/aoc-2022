use std::{env, fs};

use crate::{
    elf::Elf,
    infinite_grid::{InfiniteGrid, Position},
};

pub fn read_file_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    fs::read_to_string(path).unwrap()
}

pub fn parse_input(input: &str) -> InfiniteGrid<Elf> {
    let elfs = input.lines().enumerate().flat_map(parse_row);
    InfiniteGrid::new(elfs)
}

pub fn parse_row((row_index, line): (usize, &str)) -> impl Iterator<Item = (Position, Elf)> + '_ {
    line.chars()
        .enumerate()
        .filter_map(move |(column_index, c)| match c {
            '#' => Some((
                Position {
                    x: column_index.try_into().unwrap(),
                    y: row_index.try_into().unwrap(),
                },
                Elf::new(),
            )),
            _ => None,
        })
}
