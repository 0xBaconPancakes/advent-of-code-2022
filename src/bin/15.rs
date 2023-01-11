use nested_intervals::IntervalSet;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::newline,
    multi::many1,
    IResult,
};
use num::Integer;
use std::ops::Range;
use std::{collections::HashSet, ops::RangeInclusive};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn to_answer(&self) -> u64 {
        (self.x as u64) * 4000000 + (self.y as u64)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct DiagonalRange {
    start: Coord,
    end: Coord,
    a: i32, // 1 or -1 depending on whether the diagonal is increasing or decreasing
    b: i32, // offset from the (0, 0) coordinate
    x_range: RangeInclusive<i32>,
}

impl DiagonalRange {
    fn new(start: Coord, end: Coord) -> Self {
        if (start.x - end.x).abs() != (start.y - end.y).abs() {
            panic!("DiagonalRange must be diagonal");
        }
        // Flip the coordinates if start is to the right of end
        let (start, end) = if start.x <= end.x {
            (start, end)
        } else {
            (end, start)
        };
        let a = if start.y <= end.y { 1 } else { -1 };
        let b = start.y - a * start.x;
        let x_range = start.x..=end.x;
        Self {
            start,
            end,
            a,
            b,
            x_range,
        }
    }

    fn intersect(&self, other: &Self) -> Option<Coord> {
        if self.a == other.a {
            // Parallel
            None
        } else if self.b.is_even() != other.b.is_even() {
            // Do not intersect in a point
            None
        } else {
            let x = (other.b - self.b) / (self.a - other.a);
            let y = self.a * x + self.b;
            let coord = Coord::new(x, y);
            if self.contains(&coord) && other.contains(&coord) {
                Some(coord)
            } else {
                None
            }
        }
    }

    fn contains(&self, coord: &Coord) -> bool {
        self.x_range.contains(&coord.x) && self.a * coord.x + self.b == coord.y
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Sensor {
    sensor: Coord,
    beacon: Coord,
    range: i32,
}

impl Sensor {
    fn new(sx: i32, sy: i32, bx: i32, by: i32) -> Self {
        let sensor = Coord::new(sx, sy);
        let beacon = Coord::new(bx, by);
        if sensor == beacon {
            panic!("Sensor and beacon must be different");
        }
        Self {
            sensor,
            beacon,
            range: sensor.distance_to(&beacon),
        }
    }

    fn distance_to(&self, other: &Self) -> i32 {
        self.sensor.distance_to(&other.sensor)
    }

    fn outside_range(&self, coord: &Coord) -> bool {
        self.sensor.distance_to(coord) > self.range
    }

    fn overlap_on_y_coord(&self, target_y: i32) -> Option<Range<i32>> {
        let range = self.sensor.distance_to(&self.beacon);
        let distance_to_y = (self.sensor.y - target_y).abs();
        if distance_to_y <= range {
            let x1 = self.sensor.x - (range - distance_to_y);
            let x2 = self.sensor.x + (range - distance_to_y);
            Some(x1..x2 + 1)
        } else {
            None
        }
    }

    fn overlaps_perimeter(&self, other: &Self) -> bool {
        let completely_out_or_touching = self.distance_to(other) > self.range + other.range;
        let completely_in = self.distance_to(other) < (self.range - other.range).abs();
        !completely_out_or_touching && !completely_in
    }

    fn x_perimeter_right(&self) -> Coord {
        Coord::new(self.sensor.x + self.range + 1, self.sensor.y)
    }

    fn x_perimeter_left(&self) -> Coord {
        Coord::new(self.sensor.x - self.range - 1, self.sensor.y)
    }

    fn y_perimeter_bottom(&self) -> Coord {
        Coord::new(self.sensor.x, self.sensor.y - self.range - 1)
    }

    fn y_perimeter_top(&self) -> Coord {
        Coord::new(self.sensor.x, self.sensor.y + self.range + 1)
    }

    fn perimeter_diagonals(&self) -> Vec<DiagonalRange> {
        let left_top_diagonal = DiagonalRange::new(self.x_perimeter_left(), self.y_perimeter_top());
        let bottom_right_diagonal =
            DiagonalRange::new(self.y_perimeter_bottom(), self.x_perimeter_right());
        let left_bottom_diagonal =
            DiagonalRange::new(self.x_perimeter_left(), self.y_perimeter_bottom());
        let top_right_diagonal =
            DiagonalRange::new(self.y_perimeter_top(), self.x_perimeter_right());
        vec![
            left_top_diagonal,
            bottom_right_diagonal,
            left_bottom_diagonal,
            top_right_diagonal,
        ]
    }

    fn perimeter_intersection(&self, other: &Self) -> HashSet<Coord> {
        let self_diagonals = self.perimeter_diagonals();
        let other_diagonals = other.perimeter_diagonals();
        let mut intersections = HashSet::new();
        for self_diagonal in self_diagonals {
            for other_diagonal in &other_diagonals {
                if let Some(intersection) = self_diagonal.intersect(other_diagonal) {
                    intersections.insert(intersection);
                }
            }
        }
        intersections
    }
}

fn is_digit_minus(c: char) -> bool {
    c.is_digit(10) || c == '-'
}

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
fn parse_line(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, x) = take_while1(is_digit_minus)(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = take_while1(is_digit_minus)(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, bx) = take_while1(is_digit_minus)(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, by) = take_while1(is_digit_minus)(input)?;
    let (input, _) = newline(input)?;
    Ok((
        input,
        Sensor::new(
            x.parse().unwrap(),
            y.parse().unwrap(),
            bx.parse().unwrap(),
            by.parse().unwrap(),
        ),
    ))
}

fn parse_input(input: &str) -> Vec<Sensor> {
    many1(parse_line)(input).unwrap().1
}

pub fn part_one(input: &str, target_y: i32) -> Option<u32> {
    let sensors = parse_input(input);
    let mut min_x = sensors
        .iter()
        .map(|s| [s.sensor.x, s.beacon.x])
        .flatten()
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|s| [s.sensor.x, s.beacon.x])
        .flatten()
        .max()
        .unwrap();
    let width_x = max_x - min_x;
    min_x -= width_x;

    let beacons_on_target_y: HashSet<Coord> = sensors
        .iter()
        .filter(|s| s.beacon.y == target_y)
        .map(|s| s.beacon)
        .collect();

    let target_line_overlaps = sensors
        .iter()
        .filter_map(|s| s.overlap_on_y_coord(target_y))
        .collect::<Vec<_>>();

    let offset_overlaps = target_line_overlaps
        .iter()
        .map(|r| (r.start - min_x) as u32..(r.end - min_x) as u32)
        .collect::<Vec<_>>();

    let mut interval_set = IntervalSet::new(&offset_overlaps).ok().unwrap();
    let overlap_count = interval_set.covered_units();

    Some(overlap_count - beacons_on_target_y.len() as u32)
}

pub fn part_one_run(input: &str) -> Option<u32> {
    part_one(input, 2000000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sensors = parse_input(input);
    let mut checked_coords: HashSet<Coord> = HashSet::new();

    for i in 0..sensors.len() {
        let i_sensor = &sensors[i].clone();
        println!("Checking sensor {}, {:?}", i, i_sensor);

        for j in (i + 1)..sensors.len() {
            let j_sensor = &sensors[j].clone();
            if !i_sensor.overlaps_perimeter(j_sensor) {
                continue;
            }

            for k in (j + 1)..sensors.len() {
                let condition = i == 3 && j == 6 && k == 9;
                if condition {
                    println!("Checking sensor {}, {:?}", k, sensors[k]);
                }
                let k_sensor = &sensors[k].clone();
                if !i_sensor.overlaps_perimeter(k_sensor) {
                    continue;
                }
                // J and K must be only touching
                if j_sensor.distance_to(&k_sensor) != j_sensor.range + k_sensor.range + 2 {
                    continue;
                }
                let ij_intersection = i_sensor.perimeter_intersection(j_sensor);
                let ik_intersection = i_sensor.perimeter_intersection(k_sensor);
                let target_coord_set = &ij_intersection & &ik_intersection;
                if target_coord_set.len() == 1 {
                    let target_coord = target_coord_set.iter().next().unwrap();
                    if checked_coords.contains(target_coord) {
                        continue;
                    }

                    if sensors
                        .iter()
                        .all(|s| s.outside_range(target_coord))
                    {
                        println!("Found target: {:?}", target_coord);
                        return Some(target_coord.to_answer());
                    } else {
                        checked_coords.insert(target_coord.clone());
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one_run, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n";
        let (input, sensor) = parse_line(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(sensor, Sensor::new(2, 18, -2, 15));
    }

    #[test]
    fn test_distance() {
        let a = Coord::new(0, 11);
        let b = Coord::new(2, 10);
        assert_eq!(a.distance_to(&b), 3);
    }

    #[test]
    fn test_diagonals() {
        let sensor = Sensor::new2(12, 14, 4);
        let diagonals = sensor.perimeter_diagonals();
        let target_diagonals = vec![
            DiagonalRange::new(Coord::new(7, 14), Coord::new(12, 19)),
            DiagonalRange::new(Coord::new(17, 14), Coord::new(12, 9)),
            DiagonalRange::new(Coord::new(7, 14), Coord::new(12, 9)),
            DiagonalRange::new(Coord::new(17, 14), Coord::new(12, 19)),
        ];
        assert_eq!(diagonals, target_diagonals);
    }

    #[test]
    fn test_intersect() {
        let range1 = DiagonalRange::new(Coord::new(0, 0), Coord::new(4, 4));
        let range2 = DiagonalRange::new(Coord::new(2, 2), Coord::new(6, 6));
        let range3 = DiagonalRange::new(Coord::new(0, 0), Coord::new(4, -4));
        let range4 = DiagonalRange::new(Coord::new(-2, 2), Coord::new(2, -2));
        let range5 = DiagonalRange::new(Coord::new(-1, 2), Coord::new(2, -1));

        assert_eq!(range1.intersect(&range2), None);
        assert_eq!(range1.intersect(&range3), Some(Coord::new(0, 0)));
        assert_eq!(range1.intersect(&range4), Some(Coord::new(0, 0)));
        assert_eq!(range1.intersect(&range5), None);
        assert_eq!(range4.intersect(&range5), None);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
