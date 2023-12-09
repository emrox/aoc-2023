advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<i32>> = input.lines().map(|line| line.split(' ').map(|n| n.parse::<i32>().unwrap()).collect()).collect();
    let mut result: i32 = 0;

    for line in &lines {
        let mut line_results: Vec<Vec<i32>> = [line.clone()].to_vec();

        while !line_results[line_results.len() - 1].iter().all(|n| n == &0) {
            let mut current_line_result: Vec<i32> = [].to_vec();
            let calc_line = &line_results[line_results.len() - 1];

            for number_pos in 0..(calc_line.len()-1) {
                current_line_result.push(calc_line[number_pos + 1] - calc_line[number_pos]);
            }

            line_results.push( current_line_result);
        }

        let mut last_result_number: i32 = 0;
        for line_number in 1..line_results.len() {
            let reverse_line_number = line_results.len() - line_number - 1;

            last_result_number = line_results[reverse_line_number + 1].last().unwrap() + line_results[reverse_line_number].last().unwrap();

            line_results[reverse_line_number].push(last_result_number);
        }

        result += last_result_number;
    }

    Some(result as u32)
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
