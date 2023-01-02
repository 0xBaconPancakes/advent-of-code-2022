struct Step {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Step>) {
    let init_max_stack_height = input
    .lines()
    .position(|line| line.chars().any(|c| c.is_digit(10)))
    .unwrap();
    let stack_count = input
    .lines()
    .nth(init_max_stack_height)
    .unwrap()
    .to_string()
    .split_whitespace()
    .count();
    let mut stacks = vec![Vec::new(); stack_count];
    for line in input.lines().take(init_max_stack_height) {
        for (index, crate_name) in line.chars().skip(1).step_by(4).enumerate() {
            if crate_name != ' ' {
                stacks[index].push(crate_name);
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    let steps = input.lines().skip(init_max_stack_height + 2).map(|line| {
        let mut words = line.split_whitespace().skip(1).step_by(2);
        Step {
            count: words.next().unwrap().parse().unwrap(),
            from: words.next().unwrap().parse().unwrap(),
            to: words.next().unwrap().parse().unwrap(),
        }
    })
    .collect();
    (stacks, steps)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, steps) = parse_input(input);
    for step in steps {
        for _ in 0..step.count {
            let crate_name = stacks[step.from - 1].pop().unwrap();
            stacks[step.to - 1].push(crate_name);
        }
    }
    stacks.iter().map(|stack| stack.last()).collect()
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, steps) = parse_input(input);
    for step in steps {
        let stack = &mut stacks[step.from - 1];
        let stack_len = stack.len();
        let crates = stack.split_off(stack_len - step.count);
        stacks[step.to - 1].extend(crates);
    }
    stacks.iter().map(|stack| stack.last()).collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
