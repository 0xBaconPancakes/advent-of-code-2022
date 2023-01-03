fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

fn detect_duplicates(input: &str, window_length: usize) -> Option<u32> {
    let binding = input.chars().collect::<Vec<char>>();
    let windows = binding.windows(window_length);
    for (i, window) in windows.enumerate() {
        if unique(window.iter().collect::<String>().as_str()).is_none() {
            return Some(i as u32 + window_length as u32);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    detect_duplicates(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    detect_duplicates(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));

        let input_2 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(part_one(&input_2), Some(5));

        let input_3 = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        assert_eq!(part_one(&input_3), Some(6));

        let input_4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        assert_eq!(part_one(&input_4), Some(10));

        let input_5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(part_one(&input_5), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));

        let input_2 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(part_two(&input_2), Some(23));

        let input_3 = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        assert_eq!(part_two(&input_3), Some(23));

        let input_4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        assert_eq!(part_two(&input_4), Some(29));

        let input_5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(part_two(&input_5), Some(26));
    }
}
