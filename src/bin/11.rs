use std::collections::VecDeque;

use itertools::Itertools;

enum Operation {
    Add(Term, Term),
    Multiply(Term, Term),
}

enum Term {
    Constant(u64),
    Old,
}

impl Term {
    fn evaluate(&self, old: &u64) -> u64 {
        match self {
            Term::Constant(c) => c.clone(),
            Term::Old => old.clone(),
        }
    }
}

impl Operation {
    fn evaluate(&self, old: &u64) -> u64 {
        match self {
            Operation::Add(l, r) => l.evaluate(old) + r.evaluate(old),
            Operation::Multiply(l, r) => l.evaluate(old) * r.evaluate(old),
        }
    }
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    receiver_idxs: (usize, usize),
    inspected_count: u64,
}

struct InspectionResult {
    new_item: u64,
    target_monkey: usize,
}

impl Monkey {
    // Example input:
    //
    // Monkey 0:
    // Starting items: 79, 98
    // Operation: new = old * 19
    // Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3
    fn new(monkey_input: &str) -> Self {
        let lines: Vec<String> = monkey_input.split('\n').map(|s| s.to_string()).collect();
        assert!(lines.len() == 7);
        let mut items = VecDeque::new();
        for item in lines[1]
            .split("Starting items: ")
            .nth(1)
            .unwrap()
            .split(", ")
        {
            items.push_back(item.parse().unwrap());
        }
        let mut operation_line = lines[2]
            .split("Operation: new = ")
            .nth(1)
            .unwrap()
            .split_whitespace();
        let left_operand = operation_line.next().unwrap();
        let left_term = if left_operand == "old" {
            Term::Old
        } else {
            Term::Constant(left_operand.parse().unwrap())
        };
        let operation_unparsed = operation_line.next().unwrap();
        let right_operand = operation_line.next().unwrap();
        let right_term = if right_operand == "old" {
            Term::Old
        } else {
            Term::Constant(right_operand.parse().unwrap())
        };
        let operation = match operation_unparsed {
            "+" => Operation::Add(left_term, right_term),
            "*" => Operation::Multiply(left_term, right_term),
            _ => panic!("Unknown operator: {}", operation_unparsed),
        };
        let test_divisor = lines[3]
            .split("Test: divisible by ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let monkey_throw_if_true = lines[4]
            .split("    If true: throw to monkey ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let monkey_throw_if_false = lines[5]
            .split("    If false: throw to monkey ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        Self {
            items,
            operation: operation,
            divisor: test_divisor,
            receiver_idxs: (monkey_throw_if_true, monkey_throw_if_false),
            inspected_count: 0,
        }
    }

    fn inspect_item<R>(&mut self, item: u64, round_op: R) -> InspectionResult
    where
        R: Fn(u64) -> u64,
    {
        {
            self.inspected_count += 1;
            // println!("Monkey inspect an item with a worry level of {}.", item);
            let new_item = self.operation.evaluate(&item);
            // println!("Worry level is changed to {}.", new_item);
            let boring_item = round_op(new_item);

            // println!("Monkey gets bored with an item. Worry level is divided by 3 to {}.", lcm_item);
            let target_monkey = if boring_item % self.divisor == 0 {
                self.receiver_idxs.0
            } else {
                self.receiver_idxs.1
            };
            // println!("Item with worry level {} is thrown to monkey {}.", lcm_item, target_monkey);
            InspectionResult {
                new_item: boring_item,
                target_monkey: target_monkey,
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split('\n')
        .chunks(7)
        .into_iter()
        .map(|chunk| Monkey::new(chunk.collect::<Vec<_>>().join("\n").as_str()))
        .collect()
}

// fn print_monkey_items(monkeys: &Vec<Monkey>) {
//     monkeys.iter().enumerate().for_each(|(i, monkey)| {
//         println!("Monkey {} has {:?} items.", i, monkey.items);
//     });
// }
fn inspected_counts(monkeys: &Vec<Monkey>) -> Vec<u64> {
    let inspected_counts: Vec<u64> = monkeys
        .iter()
        .map(|monkey| monkey.inspected_count)
        .collect();
    // inspected_counts.iter().enumerate().for_each(|(i, count)| {
    //     println!("Monkey {} inspected items {} times.", i, count);
    // });
    inspected_counts
}

fn solve_input<R>(mut monkeys: Vec<Monkey>, rounds: u64, round_op: R) -> Option<u64>
where
    R: Fn(u64) -> u64,
{
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let res = monkeys[i].inspect_item(item, &round_op);
                monkeys[res.target_monkey].items.push_back(res.new_item);
            }
        }

        // print_monkey_items(&monkeys);
        // if round % 1000 == 999 {
        //     println!("Round {}.", round);
        //     inspected_counts(&monkeys);
        // }
    }

    let inspected_counts = inspected_counts(&monkeys);
    // Return multiple of the two most inspected monkeys.
    Some(inspected_counts.iter().sorted().rev().take(2).product())
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkeys = parse_input(input);
    solve_input(monkeys, 20, |x: u64| x / 3u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let monkeys = parse_input(input);

    // lcm of all monkeys divisors
    let lcm: u64 = monkeys.iter().map(|monkey| monkey.divisor).product();

    solve_input(monkeys, 10000, |x: u64| x % lcm)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2_713_310_158));
    }

    #[test]
    fn test_part_one_2() {
        let input = advent_of_code::read_file("examples", 111);
        assert_eq!(part_one(&input), Some(95_472));
    }

    #[test]
    fn test_part_two_2() {
        let input = advent_of_code::read_file("examples", 111);
        assert_eq!(part_two(&input), Some(17_926_061_332));
    }
}
