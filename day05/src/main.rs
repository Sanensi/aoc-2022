use std::{env, fs};

type Stacks = Vec<Vec<char>>;

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input = read_file_from_args();
    let (mut stacks, instructions) = parse_input(&input);
    apply_instructions(&mut stacks, &instructions);

    let top_of_stacks = read_top_of_stacks(&stacks);

    println!("{:?}", top_of_stacks);
}

fn read_file_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    fs::read_to_string(path).unwrap()
}

fn parse_input(input: &str) -> (Stacks, Vec<Instruction>) {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    (parse_stacks(stacks), parse_instructions(instructions))
}

fn parse_stacks(stacks: &str) -> Stacks {
    let mut lines = stacks.lines().rev();
    let stack_count: usize = parse_stack_count_from_footer(lines.next().unwrap());
    let mut stacks: Stacks = vec![Vec::new(); stack_count];

    for line in lines {
        for i in 0..stack_count {
            let crate_index = 4 * i + 1;
            let crate_value = line.chars().nth(crate_index).unwrap();

            if crate_value != ' ' {
                stacks[i].push(crate_value);
            }
        }
    }

    stacks
}

fn parse_stack_count_from_footer(footer: &str) -> usize {
    footer.split_whitespace().last().unwrap().parse().unwrap()
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    instructions
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line.split(" ").filter_map(|n| n.parse().ok()).collect();
            Instruction {
                count: numbers[0],
                from: numbers[1] - 1,
                to: numbers[2] - 1,
            }
        })
        .collect()
}

fn apply_instructions(stacks: &mut Stacks, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        let Instruction { count, from, to } = *instruction;

        let bottom_crate = stacks[from].len() - count;
        let crates = stacks[from].split_off(bottom_crate);
        stacks[to].extend(crates);
    }
}

fn read_top_of_stacks(stacks: &Stacks) -> String {
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}
