use std::{collections::HashMap, cmp::Reverse};

use fast_paths::InputGraph;
use nom::{
    bytes::complete::{tag, take, take_while1, is_a},
    character::complete::newline,
    multi::{many1, separated_list0},
    IResult, branch::alt,
};

fn parse_line(input: &str) -> IResult<&str, Valve> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = take(2usize)(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = take_while1(|c: char| c.is_digit(10))(input)?;
    let (input, _) = tag("; ")(input)?;
    let (input, _) = alt((tag("tunnels lead"), tag("tunnel leads")))(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, _) = alt((tag("valves"), tag("valve")))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, tunnels) = separated_list0(tag(", "), take(2usize))(input)?;
    let (input, _) = newline(input)?;
    let tunnels = tunnels.iter().map(|s| s.to_string()).collect();
    Ok((
        input,
        Valve::new(
            name.to_string(),
            flow_rate.parse().unwrap(),
            tunnels,
            0,
        ),
    ))
}

fn parse_input(input: &str) -> Vec<Valve> {
    let (input, mut valves) = many1(parse_line)(input).unwrap();
    if input.len() > 0 {
        panic!("Failed to parse input");
    }
    // Sort by flow rate descending
    valves.sort_by_key(|v| Reverse(v.flow_rate));
    // Update index
    valves.iter_mut().enumerate().for_each(|(index, valve)| {
        valve.index = index;
    });
    valves
}

#[derive(Debug, PartialEq, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
    index: usize,
}

impl Valve {
    fn new(name: String, flow_rate: u32, tunnels: Vec<String>, index: usize) -> Valve {
        Valve {
            name,
            flow_rate,
            tunnels,
            index,
        }
    }
}

fn calculate_fast_graph(valves: &[Valve]) -> Vec<Vec<usize>> {
    let mut input_graph = InputGraph::new();
    let mut name_to_index = HashMap::new();

    for (index, valve) in valves.iter().enumerate() {
        name_to_index.insert(&valve.name, index);
    }

    for (index, valve) in valves.iter().enumerate() {
        for tunnel in &valve.tunnels {
            let to_index = name_to_index.get(tunnel).unwrap();
            input_graph.add_edge(index, *to_index, 1);
        }
    }

    input_graph.freeze();
    let intermediate_graph = fast_paths::prepare(&input_graph);

    let mut fastest_paths = vec![vec![0; valves.len()]; valves.len()];
    for (from_index, from_valve) in valves.iter().enumerate() {
        for (to_index, to_valve) in valves.iter().enumerate() {
            if from_index == to_index {
                continue;
            }
            let path = fast_paths::calc_path(
                &intermediate_graph,
                name_to_index[&from_valve.name],
                name_to_index[&to_valve.name],
            );
            if let Some(path) = path {
                let weight = path.get_weight();
                fastest_paths[from_index][to_index] = weight;
            }
        }
    }

    fastest_paths
}

fn open_valve(
    minutes_remaining: u32,
    current_total_pressure_released: u32,
    current_valve: Valve,
    valves: &[Valve],
    valve_is_opened: &Vec<bool>,
    fastest_paths: &Vec<Vec<usize>>,
    level: usize,
) -> u32 {
    let mut minutes_remaining = minutes_remaining;
    let mut valve_is_opened = valve_is_opened.clone();
    // Check if we can open the valve
    if !valve_is_opened[current_valve.index] && current_valve.flow_rate == 0 || minutes_remaining == 0 {
        return current_total_pressure_released;
    }

    // Open valve
    valve_is_opened[current_valve.index] = true;
    minutes_remaining = minutes_remaining - 1;
    let current_total_pressure_released = current_total_pressure_released + current_valve.flow_rate * minutes_remaining;

    find_next_valve_to_open(minutes_remaining, current_total_pressure_released, current_valve, valves, &valve_is_opened, fastest_paths, level + 1)
}

fn find_next_valve_to_open(
    minutes_remaining: u32,
    current_total_pressure_released: u32,
    current_valve: Valve,
    valves: &[Valve],
    valve_is_opened: &Vec<bool>,
    fastest_paths: &Vec<Vec<usize>>,
    level: usize,
) -> u32 {
    // Return if we are at the end
    if minutes_remaining == 0 {
        return current_total_pressure_released;
    }

    // TODO branch and bound
    let possible_pressure_released = valves
        .iter()
        .filter(|next_valve| !valve_is_opened[next_valve.index] && next_valve.flow_rate > 0)
        .map(|next_valve| {
            let minutes_to_valve = fastest_paths[current_valve.index][next_valve.index] as u32;
            if current_valve == *next_valve || minutes_to_valve > minutes_remaining{
                return current_total_pressure_released;
            }
            // println!("{}{} -> {} in {} minutes {}", "  ".repeat(level),current_valve.name, next_valve.name, minutes_to_valve, current_total_pressure_released);
            let pressure_released = open_valve(
                minutes_remaining - minutes_to_valve,
                current_total_pressure_released,
                next_valve.clone(),
                valves,
                &valve_is_opened,
                fastest_paths,
                level + 1,
            );
            pressure_released
        })
        .max()
        .unwrap_or(current_total_pressure_released);

    possible_pressure_released
}

pub fn part_one(input: &str) -> Option<u32> {
    let minutes_remaining = 30u32;
    let current_total_pressure_released = 0u32;

    let valves = parse_input(input);
    // println!("valve names: {:?}", valves.iter().map(|valve| valve.name.to_string()).collect::<Vec<String>>());
    let fastest_paths = calculate_fast_graph(&valves);
    let valve_is_opened = vec![false; valves.len()];

    let aa_valve = valves.iter().find(|valve| valve.name == "AA").unwrap();

    Some(find_next_valve_to_open(minutes_remaining, current_total_pressure_released, aa_valve.clone(), &valves, &valve_is_opened, &fastest_paths, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

// DD minute 3: 22 * 28 = 616
// BB minute 6: 13 * 25 = 325
// JJ minute 11: 11 * 20 = 220
// HH minute 18: 22 * 12 = 264
// EE minute 21: 17 * 9 = 153
// CC minute 24: 19 * 6 = 114
// Total pressure released: 616 + 325 + 220 + 264 + 153 + 114 = 1622

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\n";
        let parsed = parse_input(&input);
        assert_eq!(
            parsed,
            vec![
                Valve::new(
                    "AA".to_string(),
                    0,
                    vec!["DD".to_string(), "II".to_string(), "BB".to_string()],
                    0,
                ),
                Valve::new(
                    "BB".to_string(),
                    13,
                    vec!["CC".to_string(), "AA".to_string()],
                    1,
                ),
            ]
        );

        let input2 = "Valve HH has flow rate=22; tunnel leads to valve GG\n";
        let parsed2 = parse_input(&input2);
        assert_eq!(
            parsed2,
            vec![
                Valve::new(
                    "HH".to_string(),
                    22,
                    vec!["GG".to_string()],
                    0,
                ),
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
