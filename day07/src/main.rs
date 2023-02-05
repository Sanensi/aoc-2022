use std::{collections::HashMap, env, fs};

const DISK_SIZE: u32 = 70_000_000;
const UPDATE_SIZE: u32 = 30_000_000;

#[derive(Debug)]
enum Command {
    CD { cwd: String },
    LS { entries: HashMap<String, DirEntry> },
}

#[derive(Debug, Clone)]
enum DirEntry {
    File(File),
    Directory(Directory),
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    entries: HashMap<String, DirEntry>,
}

#[derive(Debug)]
struct FileSystem {
    root: Directory,
    cwd: Vec<String>,
}

fn main() {
    let input = read_file_from_args();
    let commands = parse_input(&input);
    let fs = explore_file_system(&commands);
    let total_size = find_smallest_directory_to_delete_size(&fs.root);

    println!("{}", total_size);
}

fn read_file_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    fs::read_to_string(path).unwrap()
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .split("$")
        .map(str::trim)
        .filter_map(parse_command)
        .collect()
}

fn parse_command(command: &str) -> Option<Command> {
    match command.get(..2) {
        Some("cd") => Some(Command::CD {
            cwd: String::from(&command[3..]),
        }),
        Some("ls") => Some(Command::LS {
            entries: command
                .lines()
                .skip(1)
                .map(parse_entry)
                .map(|entry| (String::from(entry.get_name()), entry))
                .collect(),
        }),
        _ => None,
    }
}

fn parse_entry(line: &str) -> DirEntry {
    match line.split_once(" ").unwrap() {
        ("dir", name) => DirEntry::Directory(Directory {
            name: String::from(name),
            entries: HashMap::new(),
        }),
        (size, name) => DirEntry::File(File {
            name: String::from(name),
            size: size.parse().unwrap(),
        }),
    }
}

impl DirEntry {
    fn get_name(&self) -> &str {
        match self {
            DirEntry::Directory(Directory { name, entries: _ }) => name,
            DirEntry::File(File { name, size: _ }) => name,
        }
    }
}

fn explore_file_system(commands: &Vec<Command>) -> FileSystem {
    let mut fs = FileSystem::new();

    for command in commands {
        match command {
            Command::CD { cwd } => fs.change_dir(cwd),
            Command::LS { entries } => fs.get_current_dir().entries = entries.clone(),
        }
    }

    fs
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            cwd: vec![],
            root: Directory {
                name: String::from("/"),
                entries: HashMap::new(),
            },
        }
    }

    fn get_current_dir(&mut self) -> &mut Directory {
        let mut dir = &mut self.root;

        for path in self.cwd.iter() {
            dir = match dir.entries.get_mut(path) {
                Some(DirEntry::Directory(next_dir)) => next_dir,
                _ => unreachable!(),
            }
        }

        dir
    }

    fn change_dir(&mut self, cwd: &str) {
        match cwd {
            "/" => {
                self.cwd = vec![];
            }
            ".." => {
                self.cwd.pop();
            }
            dir_name => match self.get_current_dir().entries.get(dir_name) {
                Some(DirEntry::Directory(_)) => self.cwd.push(String::from(dir_name)),
                _ => panic!("{} in not a directory", dir_name),
            },
        }
    }
}

fn find_smallest_directory_to_delete_size(root: &Directory) -> u32 {
    let space_to_free_up = UPDATE_SIZE - (DISK_SIZE - root.get_size());

    let mut dir_candidate_sizes = root
        .depth_first_flat_map(|entry| match entry {
            DirEntry::File(_) => None,
            DirEntry::Directory(dir) => Some(dir.get_size()),
        })
        .iter()
        .filter_map(|size| size.filter(|&size| size >= space_to_free_up))
        .collect::<Vec<_>>();

    dir_candidate_sizes.sort();
    dir_candidate_sizes[0]
}

impl Directory {
    fn get_size(&self) -> u32 {
        self.entries.values().map(DirEntry::get_size).sum()
    }

    fn depth_first_flat_map<F, T>(&self, f: F) -> Vec<T>
    where
        F: Fn(&DirEntry) -> T,
    {
        self.entries
            .values()
            .flat_map(|entry| entry.depth_first_flat_map(&f))
            .collect()
    }
}

impl DirEntry {
    fn get_size(&self) -> u32 {
        match self {
            DirEntry::File(File { name: _, size }) => *size,
            DirEntry::Directory(dir) => dir.get_size(),
        }
    }

    fn depth_first_flat_map<F, T>(&self, f: &F) -> Vec<T>
    where
        F: Fn(&DirEntry) -> T,
    {
        let mut mapped_values = vec![f(self)];

        match self {
            DirEntry::File(_) => (),
            DirEntry::Directory(dir) => {
                for entry in dir.entries.values() {
                    for value in entry.depth_first_flat_map(f) {
                        mapped_values.push(value)
                    }
                }
            }
        };

        mapped_values
    }
}
