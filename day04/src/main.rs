use std::{env, fs, ops::RangeInclusive};

type PairAssignment = (RangeInclusive<u32>, RangeInclusive<u32>);
type PairAssignments = Vec<PairAssignment>;

fn main() {
    let input = read_file_from_args();
    let pair_assignments = parse_pair_assignments(&input);
    let matching_range = find_matching_ranges(&pair_assignments);

    println!("{:?}", matching_range.len());
}

fn read_file_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    fs::read_to_string(path).unwrap()
}

fn parse_pair_assignments(input: &str) -> PairAssignments {
    input.lines().map(parse_pair_assignment).collect()
}

fn parse_pair_assignment(line: &str) -> PairAssignment {
    let pair = line
        .split(',')
        .map(|range| {
            let range = range
                .split('-')
                .map(|n| n.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            RangeInclusive::new(range[0], range[1])
        })
        .collect::<Vec<_>>();

    (pair[0].clone(), pair[1].clone())
}

fn find_matching_ranges(pair_assignments: &PairAssignments) -> PairAssignments {
    pair_assignments
        .iter()
        .filter(|(range_1, range_2)| overlap(range_1, range_2))
        .map(|pair| pair.clone())
        .collect()
}

fn overlap(range_1: &RangeInclusive<u32>, range_2: &RangeInclusive<u32>) -> bool {
    (range_1.start() <= range_2.start() && range_2.start() <= range_1.end())
        || (range_2.start() <= range_1.start() && range_1.start() <= range_2.end())
}
