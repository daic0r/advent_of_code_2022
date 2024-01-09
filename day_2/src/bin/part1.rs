enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn play(&self, other: &Move) -> Option<bool> {
        match (self, other) {
            (Move::Rock, Move::Scissors) => Some(true),
            (Move::Paper, Move::Rock) => Some(true),
            (Move::Scissors, Move::Paper) => Some(true),
            (Move::Rock, Move::Paper) => Some(false),
            (Move::Paper, Move::Scissors) => Some(false),
            (Move::Scissors, Move::Rock) => Some(false),
            _ => None,
        }
    }
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Invalid move"),
        }
    }
}

fn main() {
    let contents = include_str!("../../input.txt");
    let sum = contents.lines().map(|l| {
        let mut moves = l.split_whitespace();
        let a = Move::from(moves.next().unwrap());
        let b = Move::from(moves.next().unwrap());
        (b.play(&a), b)
    }).map(|(result,shape)| shape as u32 + match result {
        Some(false) => 0,
        None => 3,
        Some(true) => 6
    }).sum::<u32>();

    println!("Sum = {sum}");
}
