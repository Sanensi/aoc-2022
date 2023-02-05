use crate::{calculate_ground_coverage, diffuse, parsing::parse_input};

const EXAMPLE_INPUT: &str = "\
....#..\n\
..###.#\n\
#...#.#\n\
.#...##\n\
#.###..\n\
##.#.##\n\
.#..#..";

#[test]
fn when_diffuse_once_on_example_input_then_receive_expected_output() {
    let mut grid = parse_input(EXAMPLE_INPUT);

    grid = diffuse(&grid, 1);

    let expected_output = "\
        .....#...\n\
        ...#...#.\n\
        .#..#.#..\n\
        .....#..#\n\
        ..#.#.##.\n\
        #..#.#...\n\
        #.#.#.##.\n\
        .........\n\
        ..#..#...";
    assert_eq!(grid.to_string(), expected_output);
}

#[test]
fn when_diffuse_twice_on_example_input_then_receive_expected_output() {
    let mut grid = parse_input(EXAMPLE_INPUT);

    grid = diffuse(&grid, 2);

    let expected_output = "\
        ......#....\n\
        ...#.....#.\n\
        ..#..#.#...\n\
        ......#...#\n\
        ..#..#.#...\n\
        #...#.#.#..\n\
        ...........\n\
        .#.#.#.##..\n\
        ...#..#....";
    assert_eq!(grid.to_string(), expected_output);
}

#[test]
fn when_diffuse_thrice_on_example_input_then_receive_expected_output() {
    let mut grid = parse_input(EXAMPLE_INPUT);

    grid = diffuse(&grid, 3);

    let expected_output = "\
        ......#....\n\
        ....#....#.\n\
        .#..#...#..\n\
        ......#...#\n\
        ..#..#.#...\n\
        #..#.....#.\n\
        ......##...\n\
        .##.#....#.\n\
        ..#........\n\
        ......#....";
    assert_eq!(grid.to_string(), expected_output);
}

#[test]
fn when_diffuse_four_times_on_example_input_then_receive_expected_output() {
    let mut grid = parse_input(EXAMPLE_INPUT);

    grid = diffuse(&grid, 4);

    let expected_output = "\
        ......#....\n\
        .....#....#\n\
        .#...##....\n\
        ..#.....#.#\n\
        ........#..\n\
        #...###..#.\n\
        .#......#..\n\
        ...##....#.\n\
        ...#.......\n\
        ......#....";
    assert_eq!(grid.to_string(), expected_output);
}

#[test]
fn when_diffuse_five_times_on_example_input_then_receive_expected_output() {
    let mut grid = parse_input(EXAMPLE_INPUT);

    grid = diffuse(&grid, 5);

    let expected_output = "\
        ......#....\n\
        ...........\n\
        .#..#.....#\n\
        ........#..\n\
        .....##...#\n\
        #.#.####...\n\
        ..........#\n\
        ...##..#...\n\
        .#.........\n\
        .........#.\n\
        ...#..#....";
    assert_eq!(grid.to_string(), expected_output);
}

#[test]
fn when_diffuse_ten_times_on_example_input_then_receive_expected_output() {
    let mut grid = parse_input(EXAMPLE_INPUT);

    grid = diffuse(&grid, 10);

    let expected_output = "\
        ......#.....\n\
        ..........#.\n\
        .#.#..#.....\n\
        .....#......\n\
        ..#.....#..#\n\
        #......##...\n\
        ....##......\n\
        .#........#.\n\
        ...#.#..#...\n\
        ............\n\
        ...#..#..#..";
    assert_eq!(grid.to_string(), expected_output);
}

#[test]
fn when_diffuse_ten_times_on_example_input_then_score_is_110() {
    let mut grid = parse_input(EXAMPLE_INPUT);

    grid = diffuse(&grid, 10);
    let score = calculate_ground_coverage(&grid);

    assert_eq!(score, 110);
}
