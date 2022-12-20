
#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Tie,
}

impl Move {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => unreachable!(),
        }
    }
}


impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    println!("{:?}, {:?}", part1(input), part2(input))
}

fn part1(input: &str) -> u32 {
   let moves: Vec<(Move, Move)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split_once(' ')
                .map(|s| (Move::from_str(s.0), Move::from_str(s.1)))
        }.unwrap())
       .collect();

   return calculate_score(moves)
}

fn part2(input: &str) -> u32 {
    let moves: Vec<(Move, Move)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split_once(' ')
                .map(|s| {
                    match s.1 {
                        "X" => {
                            let move_ = match Move::from_str(s.0) {
                                Move::Rock => Move::Scissors,
                                Move::Paper => Move::Rock,
                                Move::Scissors => Move::Paper,
                            };

                            (Move::from_str(s.0), move_)
                        }
                        "Y" => (Move::from_str(s.0), Move::from_str(s.0)),
                        "Z" => {
                          let move_ = match Move::from_str(s.0) {
                                Move::Rock => Move::Paper,
                                Move::Paper => Move::Scissors,
                                Move::Scissors => Move::Rock,
                            };
                            (Move::from_str(s.0), move_)
                        }
                        _ => unreachable!()
                    }
                })
        }.unwrap())
        .collect();

    return calculate_score(moves)
}

fn calculate_score(moves: Vec<(Move, Move)>) -> u32 {
    moves.iter()
        .map(|(l, r)| {
            let score: u32 = match (l, r) {
                (Move::Rock, Move::Scissors) | (Move::Paper, Move::Rock) | (Move::Scissors, Move::Paper)  => Outcome::value(&Outcome::Lose),
                (Move::Paper, Move::Scissors) | (Move::Scissors, Move::Rock) | (Move::Rock, Move::Paper) => Outcome::value(&Outcome::Win),
                _ => Outcome::value(&Outcome::Tie),
            };

            score + *r as u32 + 1
        }).sum()
}



#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const INPUT: &str = r#"
A Y
B X
C Z
"#;

    #[test]
    fn test_part1() {
        assert_eq!(15, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(12, part2(INPUT));
    }
}
