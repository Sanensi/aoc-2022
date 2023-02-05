use std::{env, fs};

const LOWER_CASE_A_UTF16_INDEX: u32 = 97;
const UPPER_CASE_A_UTF16_INDEX: u32 = 65;

type ElfGroupRucksacks = [String; 3];

fn main() {
    let input = read_file_from_args();
    let elf_groups = parse_elf_groups(&input);
    let common_item_types = find_common_item_types(&elf_groups);
    let priority_sum = calculate_priority_sum(&common_item_types);

    print!("{:?}", priority_sum)
}

fn read_file_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    fs::read_to_string(path).unwrap()
}

fn parse_elf_groups(input: &str) -> Vec<ElfGroupRucksacks> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .map(|elfs| [elfs[0], elfs[1], elfs[2]].map(String::from))
        .collect()
}

fn find_common_item_types(elf_groups: &Vec<ElfGroupRucksacks>) -> Vec<char> {
    elf_groups.iter().map(find_common_item_type).collect()
}

fn find_common_item_type(elf_group: &ElfGroupRucksacks) -> char {
    let [elf1, elf2, elf3] = elf_group;

    for item_type in elf1.chars() {
        if elf2.find(item_type).is_some() && elf3.find(item_type).is_some() {
            return item_type;
        }
    }

    panic!("No common item found in {:?}", elf_group)
}

fn calculate_priority_sum(common_item_types: &Vec<char>) -> u32 {
    common_item_types.iter().map(convert_char_to_priority).sum()
}

fn convert_char_to_priority(c: &char) -> u32 {
    if c.is_lowercase() {
        return c.clone() as u32 - LOWER_CASE_A_UTF16_INDEX + 1;
    } else {
        return c.clone() as u32 - UPPER_CASE_A_UTF16_INDEX + 27;
    }
}
