extern crate gcollections;
extern crate interval;

use crate::interval::Interval;
use crate::interval::ops::*;
use gcollections::ops::*;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(|line| {
            let mut intervals = line
            .split(',')
            .map(|interval| {
                let (start, end) = interval.split_at(interval.find('-').unwrap());
                Interval::new(start.parse::<u32>().unwrap(), end[1..].parse::<u32>().unwrap())
            });
            let interval_a = intervals.next().unwrap();
            let interval_b = intervals.next().unwrap();
            interval_a.is_subset(&interval_b) || interval_b.is_subset(&interval_a)
        })
        .filter(|&x| x)
        .count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(|line| {
            let mut intervals = line
            .split(',')
            .map(|interval| {
                let (start, end) = interval.split_at(interval.find('-').unwrap());
                Interval::new(start.parse::<u32>().unwrap(), end[1..].parse::<u32>().unwrap())
            });
            let interval_a = intervals.next().unwrap();
            let interval_b = intervals.next().unwrap();
            interval_a.overlap(&interval_b)
        })
        .filter(|&x| x)
        .count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
