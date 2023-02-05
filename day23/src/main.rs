use elf::Elf;
use infinite_grid::{InfiniteGrid, Position};
use parsing::{parse_input, read_file_from_args};

mod elf;
mod infinite_grid;
mod parsing;

#[cfg(test)]
mod acceptance_tests;
#[cfg(test)]
mod tests;

#[derive(Clone)]
enum Cardinal {
    North,
    South,
    West,
    East,
}

struct MoveProposal {
    from: Position,
    to: Position,
}

fn main() {
    let input = read_file_from_args();
    let grid = parse_input(&input);
    let i = diffuse_untill_there_is_nothing_else_to_diffuse(&grid);

    println!("{}", i);
}

fn diffuse_untill_there_is_nothing_else_to_diffuse(grid: &InfiniteGrid<Elf>) -> i32 {
    let mut grid = grid.clone();
    let mut snapshot = grid.to_string();
    let mut directions = [
        Cardinal::North,
        Cardinal::South,
        Cardinal::West,
        Cardinal::East,
    ]
    .iter()
    .cycle();

    let mut i = 0;
    while true {
        grid = diffuse_once(&grid, directions.clone().take(4).cloned());
        directions.next();
        i += 1;

        if snapshot == grid.to_string() {
            break;
        }

        snapshot = grid.to_string();
    }

    i
}

fn diffuse(grid: &InfiniteGrid<Elf>, times: u32) -> InfiniteGrid<Elf> {
    let mut grid = grid.clone();
    let mut directions = [
        Cardinal::North,
        Cardinal::South,
        Cardinal::West,
        Cardinal::East,
    ]
    .iter()
    .cycle();

    for _ in 0..times {
        grid = diffuse_once(&grid, directions.clone().take(4).cloned());
        directions.next();
    }

    grid
}

fn diffuse_once<Directions>(grid: &InfiniteGrid<Elf>, directions: Directions) -> InfiniteGrid<Elf>
where
    Directions: Iterator<Item = Cardinal>,
{
    let proposals = get_move_proposals(grid, &directions.collect()).collect();
    let proposals = remove_dupplicate_proposals(&proposals);
    execute_moves(grid, &proposals)
}

fn get_move_proposals<'a>(
    grid: &'a InfiniteGrid<Elf>,
    directions: &'a Vec<Cardinal>,
) -> impl Iterator<Item = MoveProposal> + 'a {
    grid.iter().filter_map(move |(position, _)| {
        if position
            .get_adjacent_positions()
            .iter()
            .all(|p| grid.get(p).is_none())
        {
            return None;
        }

        for direction in directions {
            match direction {
                Cardinal::North => {
                    if position
                        .get_northward_positions()
                        .iter()
                        .all(|p| grid.get(p).is_none())
                    {
                        return Some(MoveProposal {
                            from: position.clone(),
                            to: Position {
                                x: position.x,
                                y: position.y - 1,
                            },
                        });
                    }
                }
                Cardinal::South => {
                    if position
                        .get_southward_positions()
                        .iter()
                        .all(|p| grid.get(p).is_none())
                    {
                        return Some(MoveProposal {
                            from: position.clone(),
                            to: Position {
                                x: position.x,
                                y: position.y + 1,
                            },
                        });
                    }
                }
                Cardinal::West => {
                    if position
                        .get_westward_positions()
                        .iter()
                        .all(|p| grid.get(p).is_none())
                    {
                        return Some(MoveProposal {
                            from: position.clone(),
                            to: Position {
                                x: position.x - 1,
                                y: position.y,
                            },
                        });
                    }
                }
                Cardinal::East => {
                    if position
                        .get_eastward_positions()
                        .iter()
                        .all(|p| grid.get(p).is_none())
                    {
                        return Some(MoveProposal {
                            from: position.clone(),
                            to: Position {
                                x: position.x + 1,
                                y: position.y,
                            },
                        });
                    }
                }
            }
        }

        return None;
    })
}

fn remove_dupplicate_proposals(proposals: &Vec<MoveProposal>) -> Vec<&MoveProposal> {
    proposals
        .iter()
        .filter(|proposal_1| {
            proposals
                .iter()
                .find(|proposal_2| {
                    proposal_1.to == proposal_2.to && proposal_1.from != proposal_2.from
                })
                .is_none()
        })
        .collect()
}

fn execute_moves(grid: &InfiniteGrid<Elf>, proposals: &Vec<&MoveProposal>) -> InfiniteGrid<Elf> {
    let mut grid = grid.clone();
    for MoveProposal { from, to } in proposals {
        let elf = grid.remove(&from).unwrap();
        grid.insert(to.clone(), elf);
    }
    grid
}

fn calculate_ground_coverage(grid: &InfiniteGrid<Elf>) -> i32 {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for (Position { x, y }, _) in grid.iter() {
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

    return (max_x + 1 - min_x) * (max_y + 1 - min_y)
        - TryInto::<i32>::try_into(grid.len()).unwrap();
}
