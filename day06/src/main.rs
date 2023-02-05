use std::{env, fs};

fn main() {
    let input = read_file_from_args();
    let marker_index = find_start_of_message_marker(&input);

    println!("{}", marker_index);
}

fn read_file_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    fs::read_to_string(path).unwrap()
}

fn find_start_of_message_marker(input: &str) -> usize {
    for i in 14..input.len() {
        let rolling_window = &input[i - 14..i];

        if has_no_duplicate(rolling_window) {
            return i;
        }
    }

    panic!("No marker found for input: {}", input);
}

fn has_no_duplicate(s: &str) -> bool {
    !s.chars().enumerate().any(|(i, c)| s.find(c) != Some(i))
}

#[cfg(test)]
mod tests {
    use crate::find_start_of_message_marker;

    #[test]
    fn example_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        let marker = find_start_of_message_marker(input);

        assert_eq!(marker, 19)
    }

    #[test]
    fn example_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        let marker = find_start_of_message_marker(input);

        assert_eq!(marker, 23)
    }

    #[test]
    fn example_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";

        let marker = find_start_of_message_marker(input);

        assert_eq!(marker, 23)
    }

    #[test]
    fn example_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        let marker = find_start_of_message_marker(input);

        assert_eq!(marker, 29)
    }

    #[test]
    fn example_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        let marker = find_start_of_message_marker(input);

        assert_eq!(marker, 26)
    }
}
