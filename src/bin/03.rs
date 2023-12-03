use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let number_rgx = Regex::new(r"([0-9]+)").unwrap();
    let symbol_rgx = Regex::new(r"([^0-9\.])").unwrap();

    let mut part_number_sum: u32 = 0;

    for line_number in 0..lines.len() {
        let line = lines[line_number];
        let numbers_matches = number_rgx.find_iter(line);

        let mut symbol_positions: Vec<i32> = [].to_vec();

        for sym_line_number in (line_number as i32 - 1)..(line_number as i32 + 2) {
            if sym_line_number < 0 || sym_line_number as usize >= lines.len() {
                continue;
            }

            let sym_line = lines[sym_line_number as usize];

            let symbol_matches = symbol_rgx.find_iter(sym_line);

            for symbol_match in symbol_matches {
                symbol_positions.push(symbol_match.start() as i32);
            }
        }

        for number_match in numbers_matches {
            let box_start = if number_match.start() > 0 {number_match.start() as i32 - 1 } else { 0 };
            let box_end = number_match.end() as i32;

            let position_check = symbol_positions.iter().any(|&pos| pos >= box_start && pos <= box_end);

            if position_check {
                part_number_sum += number_match.as_str().parse::<u32>().unwrap();
            }
        }
    }

    Some(part_number_sum)
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
