use std::{env, fs};

#[derive(Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Round {
    opponent_move: Move,
    my_move: Move,
}

fn main() {
    let file_content = read_file_from_args();
    let rounds = parse_rounds(&file_content);
    let score = calculate_score(&rounds);

    print!("{:?}", score);
}

fn read_file_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    fs::read_to_string(path).unwrap()
}

fn parse_rounds(s: &str) -> Vec<Round> {
    s.lines().map(parse_round).collect()
}

fn parse_round(round: &str) -> Round {
    let (opp, result) = round.split_once(" ").unwrap();
    let opponent_move = parse_move(opp);
    let my_move = parse_my_move(result, &opponent_move);

    Round {
        opponent_move,
        my_move,
    }
}

fn parse_move(s: &str) -> Move {
    match s {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        &_ => panic!("{} is not a valid move", s),
    }
}

fn parse_my_move(result: &str, opponent_move: &Move) -> Move {
    match result {
        "X" => match opponent_move {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        "Y" => opponent_move.clone(),
        "Z" => match opponent_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        &_ => panic!("{} is not a valid result", result),
    }
}

fn calculate_score(rounds: &Vec<Round>) -> i32 {
    rounds
        .iter()
        .map(
            |Round {
                 opponent_move,
                 my_move,
             }| {
                match my_move {
                    Move::Rock => {
                        1 + (match opponent_move {
                            Move::Rock => 3,
                            Move::Paper => 0,
                            Move::Scissors => 6,
                        })
                    }
                    Move::Paper => {
                        2 + (match opponent_move {
                            Move::Rock => 6,
                            Move::Paper => 3,
                            Move::Scissors => 0,
                        })
                    }
                    Move::Scissors => {
                        3 + (match opponent_move {
                            Move::Rock => 0,
                            Move::Paper => 6,
                            Move::Scissors => 3,
                        })
                    }
                }
            },
        )
        .sum()
}
