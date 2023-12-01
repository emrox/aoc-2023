advent_of_code::solution!(1);

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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
