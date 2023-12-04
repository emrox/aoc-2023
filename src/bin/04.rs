use std::collections::HashSet;

use regex::Regex;

advent_of_code::solution!(4);

fn get_numbers_from_string(string: &str) -> Vec<u32> {
    let number_regex = Regex::new(r"([0-9]+)").unwrap();

    number_regex.find_iter(string).map(|m| { m.as_str().parse::<u32>().unwrap() }).collect::<Vec<u32>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut total_points: u32 = 0;

    for line in lines {
        let game_split = line.split(':').collect::<Vec<&str>>();
        let number_split = game_split[1].split('|').collect::<Vec<&str>>();

        let winning_numbers =get_numbers_from_string(number_split[0]);
        let game_numbers = get_numbers_from_string(number_split[1]);

        let item_set: HashSet<&u32> = winning_numbers.iter().collect();
        let difference: Vec<u32> = game_numbers.into_iter().filter(|item| item_set.contains(item)).collect();

        let matching_count = difference.len() as u32;
        let game_points = match matching_count {
            0 => continue,
            1 => 1,
            _ => 2_u32.pow(matching_count - 1)
        };

        total_points += game_points;
    }

    Some(total_points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let lines_count = lines.len() as u32;
    let mut game_copies: Vec<u32> = (0..lines_count as u32).map(|_| 1 as u32).collect();

    for line_number in 0..lines_count {
        let line = lines[line_number as usize];

        let game_split = line.split(':').collect::<Vec<&str>>();
        let number_split = game_split[1].split('|').collect::<Vec<&str>>();

        let winning_numbers = get_numbers_from_string(number_split[0]);
        let game_numbers = get_numbers_from_string(number_split[1]);

        let item_set: HashSet<&u32> = winning_numbers.iter().collect();
        let difference: Vec<u32> = game_numbers.into_iter().filter(|item| item_set.contains(item)).collect();

        let matching_count = difference.len() as u32;
        let max_add_copy_game = if matching_count > lines_count - line_number - 1 { lines_count - line_number - 1 as u32 } else { matching_count };

        for _game_number in 0..game_copies[line_number as usize] {
            for add_to_game in 0..max_add_copy_game {
                game_copies[(line_number + add_to_game + 1) as usize] += 1;
            }
        }
    }

    Some(game_copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
