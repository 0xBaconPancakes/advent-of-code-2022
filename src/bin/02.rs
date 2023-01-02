use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Ordering::Equal,
                Shape::Paper => Ordering::Less,
                Shape::Scissors => Ordering::Greater,
            },
            Shape::Paper => match other {
                Shape::Rock => Ordering::Greater,
                Shape::Paper => Ordering::Equal,
                Shape::Scissors => Ordering::Less,
            },
            Shape::Scissors => match other {
                Shape::Rock => Ordering::Less,
                Shape::Paper => Ordering::Greater,
                Shape::Scissors => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Shape {
    fn parse(shape: &str) -> Shape {
        match shape {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Invalid shape: {}", shape),
        }
    }

    fn choose_shape(opponent: Shape, round_end: &str) -> Shape {
        match round_end {
            "X" => match opponent { // lose
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            "Y" => match opponent { // draw
                Shape::Rock => Shape::Rock,
                Shape::Paper => Shape::Paper,
                Shape::Scissors => Shape::Scissors,
            },
            "Z" => match opponent { // win
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            _ => panic!("Invalid round end: {}", round_end),
        }
    }

    fn score(&self) -> u32 {
        *self as u32
    }
}

struct Round {
    opponent: Shape,
    player: Shape,
}

impl Round {
    fn round_score(&self) -> u32 {
        match self.player.cmp(&self.opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
    }

    fn score(&self) -> u32 {
        self.player.score() + self.round_score()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rounds = Vec::new();
    for line in input.lines() {
        let mut shapes = line.split_whitespace();
        let opponent = Shape::parse(shapes.next().unwrap());
        let player = Shape::parse(shapes.next().unwrap());
        rounds.push(Round {
            opponent: opponent,
            player: player,
        });
    }
    let score = rounds.iter().map(|round| round.score()).sum();
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rounds = Vec::new();
    for line in input.lines() {
        let mut shapes = line.split_whitespace();
        let opponent = Shape::parse(shapes.next().unwrap());
        let round_end = shapes.next().unwrap();
        rounds.push(Round {
            opponent: opponent,
            player: Shape::choose_shape(opponent, round_end),
        });
    }
    let score = rounds.iter().map(|round| round.score()).sum();
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_score() {
        assert_eq!(Shape::Rock.score(), 1);
        assert_eq!(Shape::Paper.score(), 2);
        assert_eq!(Shape::Scissors.score(), 3);
    }

    #[test]
    fn test_round_score() {
        assert_eq!(
            Round {
                player: Shape::Rock,
                opponent: Shape::Paper,
            }
            .round_score(),
            0
        );
        assert_eq!(
            Round {
                player: Shape::Paper,
                opponent: Shape::Scissors,
            }
            .round_score(),
            0
        );
        assert_eq!(
            Round {
                player: Shape::Scissors,
                opponent: Shape::Rock,
            }
            .round_score(),
            0
        );

        assert_eq!(
            Round {
                player: Shape::Rock,
                opponent: Shape::Rock,
            }
            .round_score(),
            3
        );
        assert_eq!(
            Round {
                player: Shape::Paper,
                opponent: Shape::Paper,
            }
            .round_score(),
            3
        );
        assert_eq!(
            Round {
                player: Shape::Paper,
                opponent: Shape::Paper,
            }
            .round_score(),
            3
        );

        assert_eq!(
            Round {
                player: Shape::Rock,
                opponent: Shape::Scissors,
            }
            .round_score(),
            6
        );
        assert_eq!(
            Round {
                player: Shape::Paper,
                opponent: Shape::Rock,
            }
            .round_score(),
            6
        );
        assert_eq!(
            Round {
                player: Shape::Scissors,
                opponent: Shape::Paper,
            }
            .round_score(),
            6
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
