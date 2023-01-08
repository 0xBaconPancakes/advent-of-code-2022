pub fn part_one(input: &str) -> Option<u32> {
    let mut value = 1;
    let mut cycle = 1;
    let mut signal_strength_sum = 0;

    let mut check_cycle = |cycle, value| {
        if cycle % 40 == 20 {
            let signal_strength = cycle * value;
            println!(
                "Cycle: {}, Value: {}, Signal Strength: {}",
                cycle, value, signal_strength
            );
            signal_strength_sum += signal_strength;
        };
    };

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let instruction = parts.next().unwrap();
        match instruction {
            "noop" => {
                cycle += 1;
            }
            "addx" => {
                let input = parts.next().unwrap().parse::<i32>().unwrap();
                cycle += 1;
                check_cycle(cycle, value);
                cycle += 1;
                value += input;
            }
            _ => {
                panic!("Invalid instruction!")
            }
        }
        check_cycle(cycle, value);
    }
    Some(signal_strength_sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sprite_position: i32 = 1;
    let mut cycle: i32 = 1;
    let mut screen = Vec::new();

    let mut draw = |cycle, sprite_position| {
        let cursor: usize = ((cycle - 1) % 40) as usize;
        // let mut sprite_viz = vec!['.'; 40];
        // if (0..39).contains(&sprite_position) {
        //     sprite_viz[sprite_position as usize] = 'X';
        //     sprite_viz[(sprite_position - 1) as usize] = 'X';
        //     sprite_viz[(sprite_position + 1) as usize] = 'X';
        // }

        // let mut cursor_viz = vec!['.'; 40];
        // cursor_viz[cursor] = 'O';

        let sprite_distance: i32 = (cursor as i32 % 40) - sprite_position;
        if sprite_distance.abs() <= 1 {
            screen.push('#');
        } else {
            screen.push('.');
        }

        // println!("Cycle: {}", cycle);
        // println!("{:?}", sprite_viz.iter().cloned().collect::<String>());
        // println!("{:?}", cursor_viz.iter().cloned().collect::<String>());
        // screen
        //     .chunks(40)
        //     .for_each(|line| println!("{:?}", line.iter().cloned().collect::<String>()));
    };

    for line in input.lines() {
        draw(cycle, sprite_position);

        let mut parts = line.split_whitespace();
        let instruction = parts.next().unwrap();
        match instruction {
            "noop" => {
                cycle += 1;
            }
            "addx" => {
                let input = parts.next().unwrap().parse::<i32>().unwrap();
                cycle += 1;
                draw(cycle, sprite_position);
                cycle += 1;
                sprite_position += input;
            }
            _ => {
                panic!("Invalid instruction!")
            }
        }
    }

    screen.iter().enumerate().for_each(|(n, pixel)| {
        print!("{}", pixel);
        if n % 40 == 39 {
            println!()
        }
    });

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
