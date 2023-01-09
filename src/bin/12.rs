use std::{collections::VecDeque, fmt, slice::Windows};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    End,
    Start,
    Unvisited,
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '╵',
            Direction::Down => '╷',
            Direction::Left => '╴',
            Direction::Right => '╶',
            Direction::End => 'E',
            Direction::Start => 'S',
            Direction::Unvisited => '.',
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn direction(&self, other: &Coord) -> Direction {
        if self.x as i32 - other.x as i32 == 1 {
            Direction::Left
        } else if self.x as i32 - other.x as i32 == -1 {
            Direction::Right
        } else if self.y as i32 - other.y as i32 == 1 {
            Direction::Up
        } else {
            Direction::Down
        }
    }

    fn estimated_distance(&self, other: &Coord) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    coord: Coord,
    elevation: u32,
    prev: Option<Coord>,
    distance_from_end: u32,
    distance_to_end_heuristic: u32,
    visited: bool,
}

impl Node {
    fn start_node(coord: Coord) -> Node {
        Node {
            coord,
            elevation: 1,
            prev: None,
            distance_from_end: u32::MAX,
            distance_to_end_heuristic: 0,
            visited: false,
        }
    }

    fn end_node(coord: Coord) -> Node {
        Node {
            coord,
            elevation: 27,
            prev: None,
            distance_from_end: 0,
            distance_to_end_heuristic: 0,
            visited: false,
        }
    }

    fn new(coord: Coord, elevation: u32) -> Node {
        Node {
            coord,
            elevation,
            prev: None,
            distance_from_end: u32::MAX,
            distance_to_end_heuristic: 0,
            visited: false,
        }
    }

    fn elevation_passable(&self, neighbor: &Node) -> bool {
        neighbor.elevation >= (self.elevation - 1)
    }
}

struct Input {
    map: Vec<Vec<Node>>,
    start: Coord,
    end: Coord,
    height: usize,
    width: usize,
}

impl Input {
    fn node(&self, coord: &Coord) -> &Node {
        &self.map[coord.y][coord.x]
    }

    fn estimated_distance(&self, coord: &Coord) -> u32 {
        let node = &self.map[coord.y][coord.x];
        let end_node = &self.map[self.end.y][self.end.x];
        let g = node.distance_from_end;
        let h = node.coord.estimated_distance(&end_node.coord);
        g + h
    }

    fn shortest_path(&mut self, first_part: bool) -> Vec<Coord> {
        // BFS
        let mut open: VecDeque<Coord> = VecDeque::new();
        open.push_back(self.end);

        while let Some(current) = open.pop_front() {
            // TODO optimize
            // let min_index = (0..(open.len()))
            //     .map(|i| (i, self.estimated_distance(&open[i])))
            //     .min_by_key(|(_, d)| *d)
            //     .unwrap()
            //     .0;

            let current_distance = self.node(&current).distance_from_end;

            {
                let mut node = &mut self.map[current.y][current.x];
                node.visited = true;

                if (first_part && current == self.start)
                    || (!first_part && node.elevation == 1)
                {
                    return self.get_path(current);
                }
               
            }

            let neighbors = self.neighbors(&current);
            for neighbor in neighbors {
                let neighbor_node = &mut self.map[neighbor.y][neighbor.x];
                if neighbor_node.distance_from_end > current_distance + 1 {
                    neighbor_node.distance_from_end = current_distance + 1;
                    neighbor_node.prev = Some(current);
                    neighbor_node.visited = true;
                    open.push_back(neighbor);
                }
            }

            // self.print_path(current);
            // self.print_visited_or_open(&open);
        }
        Vec::new()
    }

    fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        let node = &self.map[coord.y][coord.x];
        let mut neighbors: Vec<Coord> = Vec::new();
        if coord.y > 0 {
            let neighbor = &self.map[coord.y - 1][coord.x];
            if node.elevation_passable(&neighbor) {
                neighbors.push(Coord {
                    x: coord.x,
                    y: coord.y - 1,
                });
            }
        }
        if coord.y < self.height - 1 {
            let neighbor = &self.map[coord.y + 1][coord.x];
            if node.elevation_passable(&neighbor) {
                neighbors.push(Coord {
                    x: coord.x,
                    y: coord.y + 1,
                });
            }
        }
        if coord.x > 0 {
            let neighbor = &self.map[coord.y][coord.x - 1];
            if node.elevation_passable(&neighbor) {
                neighbors.push(Coord {
                    x: coord.x - 1,
                    y: coord.y,
                });
            }
        }
        if coord.x < self.width - 1 {
            let neighbor = &self.map[coord.y][coord.x + 1];
            if node.elevation_passable(&neighbor) {
                neighbors.push(Coord {
                    x: coord.x + 1,
                    y: coord.y,
                });
            }
        }

        neighbors
    }

    fn get_path(&self, target_coord: Coord) -> Vec<Coord> {
        let mut path: Vec<Coord> = Vec::new();
        let mut current: Option<Coord> = Some(target_coord);

        while current.is_some() {
            let prev_node = self.node(&current.unwrap());
            if let Some(prev) = prev_node.prev {
                path.push(prev);
            }
            current = prev_node.prev;
        }
        path
    }

    fn print_path(&self, path: &Vec<Coord>) {
        let mut directions: Vec<Vec<Direction>> =
            vec![vec![Direction::Unvisited; self.width]; self.height];

        let end_coord = path.last().unwrap();
        directions[end_coord.y][end_coord.x] = Direction::End;
        let start_coord = path.first().unwrap();
        directions[start_coord.y][start_coord.x] = Direction::Start;

        for w in path[..].windows(2) {
            let current = w[0];
            let prev = w[1];
            let direction = current.direction(&prev);
            directions[current.y][current.x] = direction;
        }

        // print directions
        for row in directions {
            for direction in row {
                print!("{}", direction);
            }
            println!();
        }
        println!();
    }
}

fn parse_input(input: &str) -> Input {
    let mut map: Vec<Vec<Node>> = Vec::new();
    let mut start: Coord = Coord { x: 0, y: 0 };
    let mut end: Coord = Coord { x: 0, y: 0 };

    input.lines().for_each(|line| {
        let mut row: Vec<Node> = Vec::new();
        line.chars().for_each(|c| {
            if c == 'S' {
                let start_coord = Coord {
                    x: row.len(),
                    y: map.len(),
                };
                let node = Node::start_node(start_coord);
                start = node.coord;
                row.push(node);
            } else if c == 'E' {
                let end_coord = Coord {
                    x: row.len(),
                    y: map.len(),
                };
                let node = Node::end_node(end_coord);
                end = end_coord;
                row.push(node);
            } else {
                let coord = Node::new(
                    Coord {
                        x: row.len(),
                        y: map.len(),
                    },
                    c as u32 - 'a' as u32 + 1,
                );
                row.push(coord);
            }
        });
        map.push(row);
    });
    let height = map.len();
    let width = map[0].len();

    // Update estimated distance to end
    for mut node in map.iter_mut().flatten() {
        node.distance_to_end_heuristic = node.coord.estimated_distance(&end);
    }

    Input {
        map,
        start,
        end,
        height,
        width,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = parse_input(input);
    let shortest_path = input.shortest_path(true);
    input.print_path(&shortest_path);
    Some(shortest_path.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = parse_input(input);
    let shortest_path = input.shortest_path(false); 
    input.print_path(&shortest_path);
    Some(shortest_path.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
