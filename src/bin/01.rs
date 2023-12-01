advent_of_code::solution!(1);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut calibration_sum: i32 = 0;
    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        let numbers: Vec<&str> = line.matches(char::is_numeric).collect();

        if numbers.is_empty() {
                continue;
        }

        let calibration_value = [numbers[0], numbers.last().unwrap()].join("").parse::<i32>().unwrap();

        calibration_sum += calibration_value;
    }

    Some(calibration_sum as u32)
}

fn word_to_number(word: &str) -> &str {
    match word {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => word,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calibration_sum: u32 = 0;
    let number_matcher = Regex::new(r"^([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        let mut str_numbers:Vec<&str> = [].to_vec();

        // we cannot just match regex since numbers like "seveneight" would just match "seven", but we need "seven" and "eight"
        for start_pos in 0..line.len() {
            if number_matcher.is_match(line.split_at(start_pos).1) {
                str_numbers.push(number_matcher.find(line.split_at(start_pos).1).unwrap().as_str());
            }
        }

        let calibration_value = [
            word_to_number(str_numbers[0]),
            word_to_number(str_numbers.last().unwrap())
        ].join("").parse::<u32>().unwrap();

        calibration_sum += calibration_value;
    }

    Some(calibration_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
