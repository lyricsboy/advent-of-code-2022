use std::io::{BufRead, Lines};

enum RPSMove {
    Rock,
    Paper,
    Scissors
}

fn decode_move(c: char) -> Option<RPSMove> {
    match c {
        'A' => Some(RPSMove::Rock),
        'B' => Some(RPSMove::Paper),
        'C' => Some(RPSMove::Scissors),
        'X' => Some(RPSMove::Rock),
        'Y' => Some(RPSMove::Paper),
        'Z' => Some(RPSMove::Scissors),
        _ => None
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
        let mut moves = line_str.split_whitespace().map(|encoded_move| {
            let encoded_char = encoded_move.chars().nth(0).unwrap();
            decode_move(encoded_char).unwrap()
        });
        let their_move = moves.nth(0);
        let your_move = moves.nth(0);
        score + rps_round_score(their_move.unwrap(), your_move.unwrap())
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
        assert_eq!(score, 15);
    }
}