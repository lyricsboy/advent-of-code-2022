use std::io::{BufRead, Lines};

#[derive(Copy, Clone)]
enum RPSMove {
    Rock,
    Paper,
    Scissors
}

enum RPSResult {
    Win,
    Lose,
    Draw
}

fn decode_move(c: char) -> Option<RPSMove> {
    match c {
        'A' => Some(RPSMove::Rock),
        'B' => Some(RPSMove::Paper),
        'C' => Some(RPSMove::Scissors),
        _ => None
    }
}

fn decode_result(c: char) -> Option<RPSResult> {
    match c {
        'X' => Some(RPSResult::Lose),
        'Y' => Some(RPSResult::Draw),
        'Z' => Some(RPSResult::Win),
        _ => None
    }
}

fn your_move_for(their_move: RPSMove, result: RPSResult) -> RPSMove {
    match result {
        RPSResult::Win =>  match their_move {
            RPSMove::Rock => RPSMove::Paper,
            RPSMove::Paper => RPSMove::Scissors,
            RPSMove::Scissors => RPSMove::Rock,
        },
        RPSResult::Lose => match their_move {
            RPSMove::Rock => RPSMove::Scissors,
            RPSMove::Paper => RPSMove::Rock,
            RPSMove::Scissors => RPSMove::Paper,
        },
        RPSResult::Draw => their_move,
    }
}

fn rps_round_score(their_move: RPSMove, your_move: RPSMove) -> i32 {
    let move_score = match your_move {
        RPSMove::Rock => 1,
        RPSMove::Paper => 2,
        RPSMove::Scissors => 3,
    };
    let round_score = match (their_move, your_move) {
        // draw
        (RPSMove::Rock, RPSMove::Rock) | (RPSMove::Paper, RPSMove::Paper) | (RPSMove::Scissors, RPSMove::Scissors) => 3,
        // they win
        (RPSMove::Rock, RPSMove::Scissors) | (RPSMove::Paper, RPSMove::Rock) | (RPSMove::Scissors, RPSMove::Paper) => 0,
        // you win
        (RPSMove::Rock, RPSMove::Paper) | (RPSMove::Paper, RPSMove::Scissors) | (RPSMove::Scissors, RPSMove::Rock) => 6,
    };
    return move_score + round_score;
}

pub(crate) fn rps_score<T:BufRead>(iterator: &mut Lines<T>) -> i32 {
    return iterator.fold(0, |score: i32, line: Result<String, std::io::Error>| -> i32 {
        let line_str = line.unwrap();
        let mut game_strategy = line_str.split_whitespace().map(|encoded_move|
            encoded_move.chars().nth(0).unwrap()
        );
        let their_move = game_strategy.nth(0)
            .map(|encoded_move| decode_move(encoded_move) )
            .flatten().expect("Could not decode their move");
        let your_move = game_strategy.nth(0)
            .map(|encoded_result| decode_result(encoded_result) ).flatten()
            .map(|result| your_move_for(their_move, result))
            .expect("Could not determine your move");

        score + rps_round_score(their_move, your_move)
    });
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const SAMPLE_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn rps_score_with_sample_input() {
        let input = SAMPLE_INPUT.to_string(); 
        let buf = BufReader::new(input.as_bytes());
        let score = rps_score(&mut buf.lines());
        assert_eq!(score, 12);
    }
}