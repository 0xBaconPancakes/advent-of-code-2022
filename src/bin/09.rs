use std::collections::HashSet;

#[derive(Clone)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new() -> Self {
        Knot { x: 0, y: 0 }
    }

    fn move_unit(&mut self, direction: char) {
        match direction {
            'U' => self.y += 1,
            'D' => self.y -= 1,
            'L' => self.x -= 1,
            'R' => self.x += 1,
            _ => panic!("Invalid direction"),
        }
    }

    fn move_after_step(&mut self, other: Knot) {
        if self.x < other.x - 1 {
            self.x = other.x - 1;
            if self.y < other.y {
                self.y += 1;
            } else if self.y > other.y {
                self.y -= 1;
            }
        } else if self.x > other.x + 1 {
            self.x = other.x + 1;
            if self.y < other.y {
                self.y += 1;
            } else if self.y > other.y {
                self.y -= 1;
            }
        } else if self.y < other.y - 1 {
            self.y = other.y - 1;
            if self.x < other.x {
                self.x += 1;
            } else if self.x > other.x {
                self.x -= 1;
            }
        } else if self.y > other.y + 1 {
            self.y = other.y + 1;
            if self.x < other.x {
                self.x += 1;
            } else if self.x > other.x {
                self.x -= 1;
            }
        }
    }
}

struct Rope {
    knots: Vec<Knot>,
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(knot_count: usize) -> Self {
        Rope {
            knots: vec![Knot::new(); knot_count],
            visited: HashSet::new(),
        }
    }

    fn move_unit(&mut self, direction: char) {
        self.knots[0].move_unit(direction);
        for i in 1..self.knots.len() {
            let other = self.knots[i - 1].clone();
            self.knots[i].move_after_step(other);
        }
        self.visited.insert((
            self.knots[self.knots.len() - 1].x,
            self.knots[self.knots.len() - 1].y,
        ));
    }

    fn move_steps(&mut self, direction: char, steps: u32) {
        for _ in 0..steps {
            self.move_unit(direction);
        }
    }

    fn print(&self, knots: bool, visited: bool) {
        let mut x_coords: Vec<i32> = self.visited.iter().map(|(x, _)| x.clone()).collect();
        x_coords.append(&mut self.knots.iter().map(|k| k.x).collect());
        let mut y_coords: Vec<i32> = self.visited.iter().map(|(_, y)| y.clone()).collect();
        y_coords.append(&mut self.knots.iter().map(|k| k.y).collect());
        let max_x = x_coords.iter().max().unwrap();
        let min_x = x_coords.iter().min().unwrap();
        let max_y = y_coords.iter().max().unwrap();
        let min_y = y_coords.iter().min().unwrap();
        let mut field =
            vec![vec!['.'; (*max_x - *min_x + 1) as usize]; (*max_y - *min_y + 1) as usize];
        field[-min_y as usize][-min_x as usize] = 's';
        if knots {
            for (n, knot) in self.knots.iter().enumerate().rev() {
                let marker: char = if n == 0 {
                    'H'
                } else {
                    (n as u8 + '0' as u8) as char
                };
                field[(knot.y - min_y) as usize][(knot.x - min_x) as usize] = marker;
            }
        }
        if visited {
            for (x, y) in &self.visited {
                field[(y - min_y) as usize][(x - min_x) as usize] = '#';
            }
        }
        for row in field.iter().rev() {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }
}

fn process_input(input: &str, knot_count: usize) -> Option<u32> {
    let mut rope = Rope::new(knot_count);
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap().chars().next().unwrap();
        let steps = parts.next().unwrap().parse::<u32>().unwrap();
        rope.move_steps(dir, steps);
        // rope.print(true, false);
        // println!();
    }
    // rope.print(false, true);
    Some(rope.visited.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    process_input(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    process_input(input, 10)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 99);
        assert_eq!(part_two(&input), Some(36));
    }
}
