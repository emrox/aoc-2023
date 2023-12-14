advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let field: Vec<Vec<char>> = input.lines().map(|l| { l.chars().collect() }).collect();
    let mut tilted_field: Vec<Vec<char>> = field.iter().map(
        |row| {
            row.iter().map(|c| return if c == &'O' { '.' } else { c.clone() }).collect()
        }
    ).collect();

    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] == 'O' {
                let mut new_row = row;
                while new_row > 0 && tilted_field[new_row - 1][col] == '.' {
                    new_row -= 1;
                }
                tilted_field[new_row][col] = 'O';
            }
        }
    }

    let row_count = tilted_field.len();
    let mut load: u32 = 0;

    for row in 0..row_count {
        let occurences = tilted_field[row].iter().filter(|c| c == &&'O').count();
        let line_load = occurences * (row_count - row);

        load = load + (line_load as u32);
    }

    Some(load)
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
