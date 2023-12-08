use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let map_re = Regex::new(r"([A-Z]+)").unwrap();

    let directions = lines[0].chars().collect::<Vec<char>>();
    let mut mapping: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines[2..lines.len()].iter() {
        let parts = map_re.find_iter(line).map(|m| { m.as_str() }).collect::<Vec<&str>>();

        mapping.insert(&parts[0], vec![&parts[1], &parts[2]]);
    }

    let mut steps = 0;
    let mut found_exit = false;
    let direction_count = directions.len();
    let mut current_position = "AAA";

    while !found_exit {
        let direction = directions[steps % direction_count];
        steps += 1;

        let next_positions = mapping.get(current_position).unwrap()[if direction == 'L' { 0 } else { 1 }];

        current_position = next_positions;
        if current_position == "ZZZ" {
            found_exit = true;
        }
    }

    Some(steps as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
