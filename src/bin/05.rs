use regex::Regex;

advent_of_code::solution!(5);

fn get_numbers_from_string(rgx: &Regex, string: &str) -> Vec<u64> {
    rgx.find_iter(string).map(|m| { m.as_str().parse::<u64>().unwrap() }).collect::<Vec<u64>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let number_regex = Regex::new(r"([0-9]+)").unwrap();
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let seeds: Vec<u64> = get_numbers_from_string(&number_regex, parts[0]);
    let mut min_seed_position: u64 = seeds[0];

    for seed in seeds {
        let mut seed_position: u64 = seed;

        for part_index in 1..parts.len() {
            let part_lines: Vec<&str> = parts[part_index].lines().collect();

            for part_line_nr in 1..part_lines.len() {
                let part_line_numbers = get_numbers_from_string(&number_regex, part_lines[part_line_nr]);
                let destination_start = part_line_numbers[0];
                let source_start = part_line_numbers[1];
                let range = part_line_numbers[2];

                if seed_position >= source_start && seed_position < (source_start + range) {
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

fn build_part_line_cache(number_regex: &Regex, lines: Vec<&str>) -> Vec<Vec<u64>> {
    let mut line_cache: Vec<Vec<u64>> = [].to_vec();

    for line in lines {
        line_cache.push(get_numbers_from_string(&number_regex, line));
    }

    line_cache
}

pub fn part_two(input: &str) -> Option<u64> {
    let number_regex = Regex::new(r"([0-9]+)").unwrap();
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let seed_numbers: Vec<u64> = get_numbers_from_string(&number_regex, parts[0]);
    let mut min_seed_position: u64 = seed_numbers[0];

    let mut part_line_cache: Vec<Vec<Vec<u64>>> = [[].to_vec()].to_vec();
    for part_index in 1..parts.len() {
        part_line_cache.push(build_part_line_cache(&number_regex, parts[part_index].lines().collect::<Vec<&str>>()));
    }

    for seed_number_index in 0..(seed_numbers.len() / 2) {
        let base_seed_index = seed_number_index * 2;
        let seed_start = seed_numbers[base_seed_index];
        let seed_range = seed_numbers[base_seed_index + 1];

        for seed in seed_start..(seed_start + seed_range) {
            let mut seed_position: u64 = seed;

            for part_index in 1..parts.len() {
                for part_line_nr in 1..part_line_cache[part_index].len() {
                    let part_line_numbers = &part_line_cache[part_index][part_line_nr];
                    let destination_start = part_line_numbers[0];
                    let source_start = part_line_numbers[1];
                    let range = part_line_numbers[2];

                    if seed_position >= source_start && seed_position < (source_start + range) {
                        seed_position = (seed_position as i64 + (destination_start as i64 - source_start as i64) as i64) as u64;
                        break;
                    }
                }
            }

            if seed_position < min_seed_position {
                min_seed_position = seed_position;
            }
        }
    }

    Some(min_seed_position)
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
        assert_eq!(result, Some(46));
    }
}
