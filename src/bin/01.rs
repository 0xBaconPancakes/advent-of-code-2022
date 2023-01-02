#[derive(Default)]
struct TopElves {
    first: u32,
    second: u32,
    third: u32,
}

impl TopElves {
    fn update(&mut self, new_elf: u32) {
        if new_elf > self.first {
            self.third = self.second;
            self.second = self.first;
            self.first = new_elf;
        } else if new_elf > self.second {
            self.third = self.second;
            self.second = new_elf;
        } else if new_elf > self.third {
            self.third = new_elf;
        }
    }

    fn sum(&self) -> u32 {
        self.first + self.second + self.third
    }
}

fn parse_input(input: &str) -> TopElves {
    let mut elves: TopElves = Default::default();
    let mut curr_elf_calories = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves.update(curr_elf_calories);
            curr_elf_calories = 0;
        } else {
            curr_elf_calories += line.parse::<u32>().unwrap();
        }
    }
    elves.update(curr_elf_calories);
    elves
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves: TopElves = parse_input(input);
    Some(elves.first)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves: TopElves = parse_input(input);
    Some(elves.sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
