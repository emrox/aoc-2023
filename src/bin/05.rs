use regex::Regex;

advent_of_code::solution!(5);

fn get_numbers_from_string(string: &str) -> Vec<u64> {
    let number_regex = Regex::new(r"([0-9]+)").unwrap();

    number_regex.find_iter(string).map(|m| { m.as_str().parse::<u64>().unwrap() }).collect::<Vec<u64>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let seeds: Vec<u64> = get_numbers_from_string(parts[0]);
    let mut min_seed_position: u64 = seeds[0];

    for seed in seeds {
        let mut seed_position: u64 = seed;

        for part_index in 1..parts.len() {
            let part_lines: Vec<&str> = parts[part_index].lines().collect();

            for part_line_nr in 1..part_lines.len() {
                let part_line_numbers = get_numbers_from_string(part_lines[part_line_nr]);
                let destination_start = part_line_numbers[0];
                let source_start = part_line_numbers[1];
                let range = part_line_numbers[2];

                if (source_start..(source_start + range)).contains(&seed_position) {
                    seed_position = (seed_position as i64 + (destination_start as i64 - source_start as i64) as i64) as u64;
                    break;
                }
            }
        }

        if seed_position < min_seed_position {
            min_seed_position = seed_position;
        }
    }

    Some(min_seed_position)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
