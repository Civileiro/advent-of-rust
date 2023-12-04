#![allow(dead_code)]

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum RPSResult {
    Win,
    Draw,
    Loss,
}

impl Move {
    pub fn from_str(c: &str) -> Option<Move> {
        match c {
            "A" | "X" => Some(Move::Rock),
            "B" | "Y" => Some(Move::Paper),
            "C" | "Z" => Some(Move::Scissors),
            _ => None,
        }
    }
    pub fn vs(&self, other: &Move) -> RPSResult {
        match self {
            Move::Rock => match other {
                Move::Rock => RPSResult::Draw,
                Move::Paper => RPSResult::Loss,
                Move::Scissors => RPSResult::Win,
            },
            Move::Paper => match other {
                Move::Rock => RPSResult::Win,
                Move::Paper => RPSResult::Draw,
                Move::Scissors => RPSResult::Loss,
            },
            Move::Scissors => match other {
                Move::Rock => RPSResult::Loss,
                Move::Paper => RPSResult::Win,
                Move::Scissors => RPSResult::Draw,
            },
        }
    }
    pub fn move_score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    pub fn move_for(&self, res: &RPSResult) -> Move {
        match self {
            Move::Rock => match res {
                RPSResult::Win => Move::Paper,
                RPSResult::Draw => Move::Rock,
                RPSResult::Loss => Move::Scissors,
            },
            Move::Paper => match res {
                RPSResult::Win => Move::Scissors,
                RPSResult::Draw => Move::Paper,
                RPSResult::Loss => Move::Rock,
            },
            Move::Scissors => match res {
                RPSResult::Win => Move::Rock,
                RPSResult::Draw => Move::Scissors,
                RPSResult::Loss => Move::Paper,
            },
        }
    }
}

impl RPSResult {
    pub fn from_str(c: &str) -> Option<RPSResult> {
        match c {
            "X" => Some(RPSResult::Loss),
            "Y" => Some(RPSResult::Draw),
            "Z" => Some(RPSResult::Win),
            _ => None,
        }
    }
    pub fn score(&self) -> i32 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Draw => 3,
            RPSResult::Loss => 0,
        }
    }
}

pub fn day2_1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let mut chars = line.split_whitespace();
            let opponent_move = Move::from_str(chars.next()?)?;
            let my_move = Move::from_str(chars.next()?)?;

            let win_score = my_move.vs(&opponent_move).score();
            let move_score = my_move.move_score();
            Some(win_score + move_score)
        })
        .sum()
}

pub fn day2_2(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let mut chars = line.split_whitespace();
            let opponent_move = Move::from_str(chars.next()?)?;
            let game_result = RPSResult::from_str(chars.next()?)?;

            let move_score = opponent_move.move_for(&game_result).move_score();
            let win_score = game_result.score();
            Some(win_score + move_score)
        })
        .sum()
}
