use std::fs::read_to_string;

pub fn main() {
    let file_path = "/home/simon/rust-projects/test/assets/advent2.txt";
    let lines = read_to_string(file_path).expect(&format!("Failed to read file {}", file_path));

    let score = lines.split_terminator('\n').fold(0, |acc, line| {
        let split = line
            .split_once(' ')
            .expect(&format!("Failed to read line {}", line));
        let opponent_move = parse_opponent_move(split.0);
        let own_move = parse_own_move(split.1);
        let own_move_score = score_move(&own_move);
        let result = calculate_result(opponent_move, own_move);
        let result_score = score_result(&result);
        return acc + own_move_score + result_score;
    });

    println!("Total score: {:?}", score);
}

enum GameMove {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Loss,
    Draw,
    Win,
}

fn calculate_result(opponent_move: GameMove, own_move: GameMove) -> GameResult {
    match (opponent_move, own_move) {
        (GameMove::Rock, GameMove::Rock) => GameResult::Draw,
        (GameMove::Rock, GameMove::Paper) => GameResult::Win,
        (GameMove::Rock, GameMove::Scissors) => GameResult::Loss,
        (GameMove::Paper, GameMove::Rock) => GameResult::Loss,
        (GameMove::Paper, GameMove::Paper) => GameResult::Draw,
        (GameMove::Paper, GameMove::Scissors) => GameResult::Win,
        (GameMove::Scissors, GameMove::Rock) => GameResult::Win,
        (GameMove::Scissors, GameMove::Paper) => GameResult::Loss,
        (GameMove::Scissors, GameMove::Scissors) => GameResult::Draw,
    }
}

fn parse_opponent_move(s: &str) -> GameMove {
    match s.as_ref() {
        "A" => GameMove::Rock,
        "B" => GameMove::Paper,
        "C" => GameMove::Scissors,
        _ => panic!("Failed to parse opponent move {:?}", s),
    }
}

fn parse_own_move(s: &str) -> GameMove {
    match s.as_ref() {
        "X" => GameMove::Rock,
        "Y" => GameMove::Paper,
        "Z" => GameMove::Scissors,
        _ => panic!("Failed to parse own move {:?}", s),
    }
}

fn score_move(m: &GameMove) -> i32 {
    match m {
        GameMove::Rock => 1,
        GameMove::Paper => 2,
        GameMove::Scissors => 3,
    }
}

fn score_result(m: &GameResult) -> i32 {
    match m {
        GameResult::Loss => 0,
        GameResult::Draw => 3,
        GameResult::Win => 6,
    }
}
