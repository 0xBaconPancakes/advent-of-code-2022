use itertools::Itertools;

trait Value {
    fn to_value(&self) -> u8;

    fn from_value(value: u8) -> Self;

    fn to_index(&self) -> usize {
        self.to_value() as usize - 1
    }

    fn from_index(index: usize) -> Self
    where
        Self: Sized,
    {
        Self::from_value(index as u8 + 1)
    }
}

impl Value for char {
    fn to_value(&self) -> u8 {
        if self.is_lowercase() {
            (u32::from(*self) - 96).try_into().unwrap()
        } else {
            (u32::from(*self) - 64 + 26).try_into().unwrap()
        }
    }

    fn from_value(value: u8) -> Self {
        if value <= 26 {
            char::from(value + 96)
        } else {
            char::from(value + 64 - 26)
        }
    }
}

struct Compartment {
    item_counts: [u32; 52],
}

impl Compartment {
    fn from_string(input: &str) -> Compartment {
        let mut item_counts = [0; 52];
        for c in input.chars() {
            item_counts[c.to_index()] += 1;
        }
        Compartment { item_counts }
    }

    fn common_value(&self, other: &Compartment) -> Option<u32> {
        for i in 0..52 {
            if self.item_counts[i] > 0 && other.item_counts[i] > 0 {
                return Some((i + 1).try_into().unwrap());
            }
        }
        None
    }

    fn common_value_3(&self, other_a: &Compartment, other_b: &Compartment) -> Option<u32> {
        for i in 0..52 {
            if self.item_counts[i] > 0 && other_a.item_counts[i] > 0 && other_b.item_counts[i] > 0 {
                return Some((i + 1).try_into().unwrap());
            }
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let priorities: Vec<u32> = input
        .lines()
        .map(|line| {
            let (half1, half2) = line.split_at(line.len() / 2);
            let first_compartment: Compartment = Compartment::from_string(half1);
            let second_compartment: Compartment = Compartment::from_string(half2);
            first_compartment.common_value(&second_compartment).unwrap()
        })
        .collect();
    Some(priorities.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let badges: Vec<u32> = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let mut lines = chunk.into_iter();
            let first_compartment: Compartment = Compartment::from_string(lines.next().unwrap());
            let second_compartment: Compartment = Compartment::from_string(lines.next().unwrap());
            let third_compartment: Compartment = Compartment::from_string(lines.next().unwrap());
            first_compartment.common_value_3(&second_compartment, &third_compartment).unwrap()
        })
        .collect();
    Some(badges.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_value() {
        assert_eq!('a'.to_value(), 1);
        assert_eq!('z'.to_value(), 26);
        assert_eq!('A'.to_value(), 27);
        assert_eq!('Z'.to_value(), 52);
    }

    #[test]
    fn test_char_from_value() {
        assert_eq!(char::from_value(1), 'a');
        assert_eq!(char::from_value(26), 'z');
        assert_eq!(char::from_value(27), 'A');
        assert_eq!(char::from_value(52), 'Z');
    }

    #[test]
    fn test_char_to_index() {
        assert_eq!('a'.to_index(), 0);
        assert_eq!('z'.to_index(), 25);
        assert_eq!('A'.to_index(), 26);
        assert_eq!('Z'.to_index(), 51);
    }

    #[test]
    fn test_char_from_index() {
        assert_eq!(char::from_index(0), 'a');
        assert_eq!(char::from_index(25), 'z');
        assert_eq!(char::from_index(26), 'A');
        assert_eq!(char::from_index(51), 'Z');
    }

    #[test]
    fn test_compartment_from_string() {
        let compartment = Compartment::from_string("abc");
        assert_eq!(compartment.item_counts[0], 1);
        assert_eq!(compartment.item_counts[1], 1);
        assert_eq!(compartment.item_counts[2], 1);
        assert_eq!(compartment.item_counts[3], 0);
    }

    #[test]
    fn test_compartment_common() {
        let compartment_a = Compartment::from_string("abc");
        let compartment_b = Compartment::from_string("adZ");
        let compartment_c = Compartment::from_string("XYZ");
        assert_eq!(
            compartment_a.common_value(&compartment_b),
            Some('a'.to_value().try_into().unwrap())
        );
        assert_eq!(compartment_a.common_value(&compartment_c), None);
        assert_eq!(
            compartment_b.common_value(&compartment_c),
            Some('Z'.to_value().try_into().unwrap())
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
