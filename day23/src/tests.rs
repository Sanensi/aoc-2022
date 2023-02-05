use crate::{
    diffuse, diffuse_once,
    elf::Elf,
    infinite_grid::{InfiniteGrid, Position},
    Cardinal,
};

type Directions = [Cardinal; 4];

const NSWE: Directions = [
    Cardinal::North,
    Cardinal::South,
    Cardinal::West,
    Cardinal::East,
];

const SWEN: Directions = [
    Cardinal::South,
    Cardinal::West,
    Cardinal::East,
    Cardinal::North,
];

#[test]
fn given_many_isolated_elves_when_diffuse_once_then_they_all_remain_where_they_are() {
    let elves = vec![
        (Position { x: 0, y: 0 }, Elf::new()),
        (Position { x: 2, y: 0 }, Elf::new()),
        (Position { x: 0, y: 2 }, Elf::new()),
    ];
    let mut grid = InfiniteGrid::new(elves.clone().into_iter());

    grid = diffuse_once(&grid, NSWE.into_iter());

    assert_eq!(grid.get(&Position { x: 0, y: 0 }), Some(&elves[0].1));
    assert_eq!(grid.get(&Position { x: 2, y: 0 }), Some(&elves[1].1));
    assert_eq!(grid.get(&Position { x: 0, y: 2 }), Some(&elves[2].1));
}

#[test]
fn given_two_adjacent_elves_on_the_x_axis_when_diffuse_once_then_they_both_move_northward() {
    let elves = vec![
        (Position { x: 0, y: 0 }, Elf::new()),
        (Position { x: 1, y: 0 }, Elf::new()),
    ];
    let mut grid = InfiniteGrid::new(elves.clone().into_iter());

    grid = diffuse_once(&grid, NSWE.into_iter());

    assert_eq!(grid.get(&Position { x: 0, y: -1 }), Some(&elves[0].1));
    assert_eq!(grid.get(&Position { x: 1, y: -1 }), Some(&elves[1].1));
}

#[test]
fn given_two_adjacent_elves_on_the_y_axis_when_diffuse_once_then_they_move_in_opposite_directions_vertically(
) {
    let elves = vec![
        (Position { x: 0, y: 0 }, Elf::new()),
        (Position { x: 0, y: 1 }, Elf::new()),
    ];
    let mut grid = InfiniteGrid::new(elves.clone().into_iter());

    grid = diffuse_once(&grid, NSWE.into_iter());

    assert_eq!(grid.get(&Position { x: 0, y: -1 }), Some(&elves[0].1));
    assert_eq!(grid.get(&Position { x: 0, y: 2 }), Some(&elves[1].1));
}

#[test]
fn given_five_adjacent_elves_in_a_cross_formation_when_diffuse_once_then_they_all_move_in_opposite_direction_except_for_the_middle_one(
) {
    let elves = vec![
        (Position { x: 0, y: 0 }, Elf::new()),
        (Position { x: -1, y: 0 }, Elf::new()),
        (Position { x: 1, y: 0 }, Elf::new()),
        (Position { x: 0, y: -1 }, Elf::new()),
        (Position { x: 0, y: 1 }, Elf::new()),
    ];
    let mut grid = InfiniteGrid::new(elves.clone().into_iter());

    grid = diffuse_once(&grid, NSWE.into_iter());

    assert_eq!(grid.get(&Position { x: 0, y: 0 }), Some(&elves[0].1));
    assert_eq!(grid.get(&Position { x: -2, y: 0 }), Some(&elves[1].1));
    assert_eq!(grid.get(&Position { x: 2, y: 0 }), Some(&elves[2].1));
    assert_eq!(grid.get(&Position { x: 0, y: -2 }), Some(&elves[3].1));
    assert_eq!(grid.get(&Position { x: 0, y: 2 }), Some(&elves[4].1));
}

#[test]
fn given_two_elves_that_propose_the_same_move_when_diffuse_once_then_they_remain_where_they_are() {
    let elves = vec![
        (Position { x: 0, y: 2 }, Elf::new()),
        (Position { x: 0, y: 1 }, Elf::new()),
        (Position { x: 0, y: -1 }, Elf::new()),
        (Position { x: 0, y: -2 }, Elf::new()),
    ];
    let mut grid = InfiniteGrid::new(elves.clone().into_iter());

    grid = diffuse_once(&grid, NSWE.into_iter());

    assert_eq!(grid.get(&Position { x: 0, y: 0 }), None);
    assert_eq!(grid.get(&Position { x: 0, y: 1 }), Some(&elves[1].1));
    assert_eq!(grid.get(&Position { x: 0, y: -1 }), Some(&elves[2].1));
}

#[test]
fn given_two_adjacent_elves_on_the_x_axis_with_swen_directions_when_diffuse_once_then_they_both_move_southward(
) {
    let elves = vec![
        (Position { x: 0, y: 0 }, Elf::new()),
        (Position { x: 1, y: 0 }, Elf::new()),
    ];
    let mut grid = InfiniteGrid::new(elves.clone().into_iter());

    grid = diffuse_once(&grid, SWEN.into_iter());

    assert_eq!(grid.get(&Position { x: 0, y: 1 }), Some(&elves[0].1));
    assert_eq!(grid.get(&Position { x: 1, y: 1 }), Some(&elves[1].1));
}

#[test]
fn given_many_isolated_elves_when_diffuse_many_times_then_they_all_remain_where_they_are() {
    let elves = vec![
        (Position { x: 0, y: 0 }, Elf::new()),
        (Position { x: 2, y: 0 }, Elf::new()),
        (Position { x: 0, y: 2 }, Elf::new()),
    ];
    let mut grid = InfiniteGrid::new(elves.clone().into_iter());

    grid = diffuse(&grid, 5);

    assert_eq!(grid.get(&Position { x: 0, y: 0 }), Some(&elves[0].1));
    assert_eq!(grid.get(&Position { x: 2, y: 0 }), Some(&elves[1].1));
    assert_eq!(grid.get(&Position { x: 0, y: 2 }), Some(&elves[2].1));
}

#[test]
fn given_two_adjacent_elves_on_the_x_axis_when_diffuse_twice_then_they_both_come_back_to_their_starting_position(
) {
    let elves = vec![
        (Position { x: 0, y: 0 }, Elf::new()),
        (Position { x: 1, y: 0 }, Elf::new()),
    ];
    let mut grid = InfiniteGrid::new(elves.clone().into_iter());

    grid = diffuse(&grid, 2);

    assert_eq!(grid.get(&Position { x: 0, y: 0 }), Some(&elves[0].1));
    assert_eq!(grid.get(&Position { x: 1, y: 0 }), Some(&elves[1].1));
}
