use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Material {
    Air,
    Sand,
    Rock,
}

#[derive(Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn down(&self) -> Self {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down_left(&self) -> Self {
        Coord {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn down_right(&self) -> Self {
        Coord {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

struct Grid {
    start: Coord,
    height: usize,
    width: usize,
    height_offset: usize,
    width_offset: usize,
    grid: Vec<Vec<Material>>,
}

fn print_grid(grid: &Vec<Vec<Material>>) {
    for row in grid.iter() {
        for col in row.iter() {
            match col {
                Material::Air => print!("."),
                Material::Sand => print!("o"),
                Material::Rock => print!("#"),
            }
        }
        println!();
    }
    println!();
}

fn get_range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

impl Grid {
    fn new(input: Vec<Vec<(&str, &str)>>, is_part_one: bool) -> Self {
        let start = Coord { x: 500, y: 0 };
        let parsed_input: Vec<Vec<(usize, usize)>> = input
            .iter()
            .map(|line| {
                line.iter()
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .collect()
            })
            .collect();
        let min_height = 0;
        let max_height = parsed_input
            .iter()
            .flatten()
            .map(|(_, y)| y)
            .max()
            .unwrap()
            .clone()
            + 2;
        let height = max_height - min_height + 1;

        let mut min_width = parsed_input
            .iter()
            .flatten()
            .map(|(x, _)| x)
            .min()
            .unwrap()
            .clone()
            - 1;
        let mut max_width = parsed_input
            .iter()
            .flatten()
            .map(|(x, _)| x)
            .max()
            .unwrap()
            .clone()
            + 1;
        min_width = if min_width < 500 - height { min_width } else { 500 - height };
        max_width = if max_width > 500 + height { max_width } else { 500 + height };
        let width = max_width - min_width + 1;

        let mut grid = vec![vec![Material::Air; width]; height];

        for line in parsed_input {
            for w in line.windows(2) {
                let (x1, y1) = w[0];
                let (x2, y2) = w[1];
                if x1 != x2 {
                    for x in get_range(x1, x2) {
                        grid[y1 - min_height][x - min_width] = Material::Rock;
                    }
                } else {
                    for y in get_range(y1, y2) {
                        grid[y - min_height][x1 - min_width] = Material::Rock;
                    }
                }
            }
        }

        if !is_part_one {
            for x in 0..width {
                grid[height - 1][x] = Material::Rock;
            }
        }

        Grid {
            start: Coord {
                x: start.x - min_width,
                y: start.y - min_height,
            },
            height,
            width,
            height_offset: min_height,
            width_offset: min_width,
            grid,
        }
    }

    fn is_empty(&self, coord: &Coord) -> bool {
        match self.grid[coord.y][coord.x] {
            Material::Air => true,
            _ => false,
        }
    }

    fn is_freefall(&self, coord: &Coord) -> bool {
        coord.y >= self.height - 1
    }

    fn generate_sand(&mut self) -> u32 {
        let mut sand_counter = 0;
        loop {
            if self.grid[self.start.y][self.start.x] == Material::Sand {
                return sand_counter;
            }

            let mut current = self.start.clone();
            loop {
                if self.is_freefall(&current) {
                    return sand_counter;
                } else if self.is_empty(&current.down()) {
                    current = current.down();
                } else if self.is_empty(&current.down_left()) {
                    current = current.down_left();
                } else if self.is_empty(&current.down_right()) {
                    current = current.down_right();
                } else {
                    self.grid[current.y][current.x] = Material::Sand;
                    break;
                }
            }
            sand_counter += 1;
            // print_grid(&self.grid);
        }
    }

    fn print(&self) {
        println!(
            "Width: {} offset {}, height: {} offset {}",
            self.width, self.width_offset, self.height, self.height_offset
        );

        print_grid(&self.grid);
    }
}

// Example input:
// 498,4 -> 498,6 -> 496,6
// 503,4 -> 502,4 -> 502,9 -> 494,9
fn parse_input(input: &str) -> IResult<&str, Vec<Vec<(&str, &str)>>> {
    separated_list0(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list0(tag(" -> "), parse_pair)(input)
}

fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(digit1, char(','), digit1)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, parsed_input) = parse_input(input).unwrap();
    let mut grid = Grid::new(parsed_input, true);
    grid.print();
    Some(grid.generate_sand())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, parsed_input) = parse_input(input).unwrap();
    let mut grid = Grid::new(parsed_input, false);
    grid.print();
    Some(grid.generate_sand())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair("1,2"), Ok(("", ("1", "2"))));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("1,2 -> 3,4"),
            Ok(("", vec![("1", "2"), ("3", "4")]))
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("1,2 -> 3,4\n5,6 -> 7,8"),
            Ok((
                "",
                vec![vec![("1", "2"), ("3", "4")], vec![("5", "6"), ("7", "8")]]
            ))
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
