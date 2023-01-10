use nom::{
    branch::alt,
    character::complete::{char, digit1, newline},
    multi::separated_list0,
    sequence::terminated,
    IResult,
};

#[derive(Debug)]
struct Pair {
    left: List,
    right: List,
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n\n", self.left, self.right)
    }
}

#[derive(Debug)]
struct Pairs {
    pairs: Vec<Pair>,
}

impl std::fmt::Display for Pairs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pair in self.pairs.iter() {
            write!(f, "{}", pair)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct List {
    elements: Vec<Listable>,
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, element) in self.elements.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", element)?;
        }
        write!(f, "]")
    }
}

impl List {
    fn compare(&self, other: &List) -> Option<bool> {
        let mut left_iter = self.elements.iter();
        let mut right_iter = other.elements.iter();
        loop {
            let (left, right) = (left_iter.next(), right_iter.next());
            match (left, right) {
                (None, Some(_)) => {
                    return Some(true);
                },
                (Some(_), None) => {
                    return Some(false);
                },
                (Some(left), Some(right)) => {
                    let res = left.compare(right);
                    if res.is_some() {
                        return res;
                    }
                },
                (None, None) => {
                    return None
                },
            }
        }
    }

    fn from_number(number: &u32) -> List {
        List {
            elements: vec![Listable::Number(*number)],
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.compare(other) {
            Some(true) => Some(std::cmp::Ordering::Less),
            Some(false) => Some(std::cmp::Ordering::Greater),
            None => Some(std::cmp::Ordering::Equal),
        }
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn print_vec_list(list: &Vec<List>) {
    for list in list {
        println!("{}", list);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Listable {
    List(List),
    Number(u32),
}

impl std::fmt::Display for Listable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Listable::List(list) => write!(f, "{}", list),
            Listable::Number(number) => write!(f, "{}", number),
        }
    }
}

impl Listable {
    fn compare(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Listable::Number(left), Listable::Number(right)) => {
                if left < right {
                    Some(true)
                } else if left > right {
                    Some(false)
                } else {
                    None
                }
            },
            (Listable::List(left), Listable::List(right)) => {
                left.compare(right)
            },
            (Listable::List(left), Listable::Number(right)) => {
                left.compare(&List::from_number(right))
            },
            (Listable::Number(left), Listable::List(right)) => {
                List::from_number(left).compare(right)
            },
        }
    }
}

impl Pair {
    fn order_check(&self) -> bool {
        self.left.compare(&self.right).unwrap()
    }
}

fn parse_number(input: &str) -> IResult<&str, Listable> {
    let (input, number) = digit1(input)?;
    let number = Listable::Number(number.parse().unwrap());
    Ok((input, number))
}

fn parse_listable(input: &str) -> IResult<&str, Listable> {
    let (input, listable) = alt((parse_list_as_listable, parse_number))(input)?;
    Ok((input, listable))
}

fn parse_list(input: &str) -> IResult<&str, List> {
    let (input, _) = char('[')(input)?;
    let (input, elements) = separated_list0(char(','), parse_listable)(input)?;
    let (input, _) = char(']')(input)?;
    Ok((input, List { elements }))
}

fn parse_list_as_listable(input: &str) -> IResult<&str, Listable> {
    let (input, list) = parse_list(input)?;
    let list = Listable::List(list);
    Ok((input, list))
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    // let (input, pair) = pair(, terminated(parse_list, newline))?;
    let (input, first) = terminated(parse_list, newline)(input)?;
    let (input, second) = terminated(parse_list, newline)(input)?;
    let pair = Pair { left: first, right: second };
    Ok((input, pair))
}

fn parse_input(input: &str) -> Pairs {
    let (input, pairs) = separated_list0(newline, parse_pair)(input).unwrap();
    if input.len() > 0 {
        panic!("Failed to parse all input");
    }
    Pairs{pairs}
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse_input(input);
    println!("{}", pairs);
    let comparisons: Vec<bool> = pairs.pairs.iter().map(|pair| pair.order_check()).collect();
    println!("{:?}", comparisons);
    Some(comparisons.into_iter().enumerate().filter_map(|(i, c)| {
        if c {
            Some(i as u32 + 1)
        } else {
            None
        }
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs = parse_input(input);
    println!("{}", pairs);
    let mut packets = pairs.pairs.into_iter().map(|pair| vec![pair.left, pair.right]).flatten().collect::<Vec<_>>();
    let additional_packet_1 = parse_list("[[2]]").unwrap().1;
    let additional_packet_2 = parse_list("[[6]]").unwrap().1;
    
    packets.push(additional_packet_1.clone());
    packets.push(additional_packet_2.clone());

    packets.sort();
    print_vec_list(&packets);

    let index_of_additional_packet_1 = packets.iter().position(|p| p == &additional_packet_1).unwrap() as u32 + 1;
    let index_of_additional_packet_2 = packets.iter().position(|p| p == &additional_packet_2).unwrap() as u32 + 1;

    Some(index_of_additional_packet_1 * index_of_additional_packet_2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let input = "123";
        let (input, number) = parse_number(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(number, Listable::Number(123));
    }

    #[test]
    fn test_parse_list() {
        let input = "[]";
        let (input, list) = parse_list(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(list, List { elements: vec![] });

        let input = "[1,2,3]";
        let (input, list) = parse_list(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            list,
            List {
                elements: vec![
                    Listable::Number(1),
                    Listable::Number(2),
                    Listable::Number(3)
                ]
            }
        );

        let input = "[1,[2,3]]";
        let (input, list) = parse_list(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            list,
            List {
                elements: vec![
                    Listable::Number(1),
                    Listable::List(List {
                        elements: vec![Listable::Number(2), Listable::Number(3)]
                    })
                ]
            }
        );
    }

    #[test]
    fn test_parse_list_as_listable() {
        let input = "[1,[2,3]]";
        let (input, list) = parse_list_as_listable(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            list,
            Listable::List(List {
                elements: vec![
                    Listable::Number(1),
                    Listable::List(List {
                        elements: vec![Listable::Number(2), Listable::Number(3)]
                    })
                ]
            })
        );
    }

    #[test]
    fn test_list_equality() {
        let list1 = parse_list("[[2]]").unwrap().1;
        let list2 = List {
            elements: vec![Listable::List(List {
                elements: vec![Listable::Number(2)]
            })]
        };
        assert_eq!(list1, list2);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
