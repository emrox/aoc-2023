use regex::Regex;

advent_of_code::solution!(6);

fn get_numbers_from_string(string: &str) -> Vec<u32> {
    let number_regex = Regex::new(r"([0-9]+)").unwrap();

    number_regex.find_iter(string).map(|m| { m.as_str().parse::<u32>().unwrap() }).collect::<Vec<u32>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let times = get_numbers_from_string(lines[0]);
    let distances = get_numbers_from_string(lines[1]);

    let mut winning_ways: u32 = 1;


    for time_index in 0..times.len() {
        let max_time = times[time_index];
        let distance = distances[time_index];

        let mut possible_wins: u32 = 0;

        for time in 1..max_time {
            let travel_time = max_time - time;
            let travel_speed = time;

            if travel_time * travel_speed > distance {
                possible_wins += 1;
            }
        }

        winning_ways = winning_ways * possible_wins;
    }

    Some(winning_ways)
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
