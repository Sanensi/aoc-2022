use std::{env, fs};

#[derive(Debug)]
struct TreeGrid<T> {
    trees: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> TreeGrid<T> {
    fn get(&self, x: usize, y: usize) -> &T {
        self.trees.get(y * self.width + x).unwrap()
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.trees.get_mut(y * self.width + x).unwrap()
    }
}

fn main() {
    let input = read_file_from_args();
    let tree_grid = parse_input(&input);
    let visibility_grid = scan_treeline(&tree_grid);
    let visible_tree_count = count_visible_tree(&visibility_grid);

    println!("{:?}", visible_tree_count);
}

fn read_file_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    fs::read_to_string(path).unwrap()
}

fn parse_input(input: &str) -> TreeGrid<i32> {
    let mut tree_grid = TreeGrid::<i32> {
        trees: vec![],
        width: input.lines().next().unwrap().len(),
        height: 0,
    };

    for row in input.lines() {
        tree_grid.height += 1;
        for tree in row.chars() {
            tree_grid
                .trees
                .push(tree.to_digit(10).unwrap().try_into().unwrap());
        }
    }

    tree_grid
}

fn scan_treeline(tree_grid: &TreeGrid<i32>) -> TreeGrid<bool> {
    let mut visibility_grid = TreeGrid {
        trees: vec![false; tree_grid.trees.len()],
        width: tree_grid.width,
        height: tree_grid.height,
    };

    for j in 0..tree_grid.height {
        let mut max_height_in_line = -1;
        for i in 0..tree_grid.width {
            if *tree_grid.get(i, j) > max_height_in_line {
                max_height_in_line = *tree_grid.get(i, j);
                *visibility_grid.get_mut(i, j) = true;
            }
        }
    }

    for j in 0..tree_grid.height {
        let mut max_height_in_line = -1;
        for i in (0..tree_grid.width).rev() {
            if *tree_grid.get(i, j) > max_height_in_line {
                max_height_in_line = *tree_grid.get(i, j);
                *visibility_grid.get_mut(i, j) = true;
            }
        }
    }

    for i in 0..tree_grid.width {
        let mut max_height_in_line = -1;
        for j in 0..tree_grid.height {
            if *tree_grid.get(i, j) > max_height_in_line {
                max_height_in_line = *tree_grid.get(i, j);
                *visibility_grid.get_mut(i, j) = true;
            }
        }
    }

    for i in 0..tree_grid.width {
        let mut max_height_in_line = -1;
        for j in (0..tree_grid.height).rev() {
            if *tree_grid.get(i, j) > max_height_in_line {
                max_height_in_line = *tree_grid.get(i, j);
                *visibility_grid.get_mut(i, j) = true;
            }
        }
    }

    visibility_grid
}

fn count_visible_tree(visibility_grid: &TreeGrid<bool>) -> usize {
    visibility_grid
        .trees
        .iter()
        .filter(|&is_visible| *is_visible)
        .count()
}
