use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let games: Vec<&str> = input.lines().collect();
    let mut possible_game_sum: u32 = 0;

    let max_red: i32 = 12;
    let max_green: i32 = 13;
    let max_blue: i32 = 14;

    let cube_matcher = Regex::new(r"([0-9]+ (red|green|blue))").unwrap();

    for game in games {
        let game_split: Vec<&str> = game.split(':').map(str::trim).collect();
        let game_number: u32 = game_split[0].split(' ').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

        let cubes: Vec<&str> = cube_matcher.find_iter(game_split[1]).map(|m| { m.as_str() }).collect();

        let mut possible_game: bool = true;

        for cube in cubes {
            let cube_split: Vec<&str> = cube.split(' ').collect();
            let cube_count: i32 = cube_split[0].parse::<i32>().unwrap();
            let cube_color: &str = cube_split[1];

            if (cube_color == "red" && cube_count > max_red)
                || (cube_color == "green" && cube_count > max_green)
                || (cube_color == "blue" && cube_count > max_blue) {
                possible_game = false;
                break;
            }
        }

        if possible_game {
            possible_game_sum += game_number;
        }
    }

    Some(possible_game_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<&str> = input.lines().collect();

    let cube_matcher = Regex::new(r"([0-9]+ (red|green|blue))").unwrap();

    let mut power: u32 = 0;

    for game in games {
        let game_split: Vec<&str> = game.split(':').map(str::trim).collect();
        let cubes: Vec<&str> = cube_matcher.find_iter(game_split[1]).map(|m| { m.as_str() }).collect();

        let mut max_red_cubes: u32 = 0;
        let mut max_green_cubes: u32 = 0;
        let mut max_blue_cubes: u32 = 0;

        for cube in cubes {
            let cube_split: Vec<&str> = cube.split(' ').collect();
            let cube_count: u32 = cube_split[0].parse::<u32>().unwrap();
            let cube_color: &str = cube_split[1];

            if cube_color == "red" && cube_count > max_red_cubes {
                max_red_cubes = cube_count;
            } else if cube_color == "green" && cube_count > max_green_cubes {
                max_green_cubes = cube_count;
            } else if cube_color == "blue" && cube_count > max_blue_cubes {
                max_blue_cubes = cube_count;
            }
        }

        power += max_red_cubes * max_green_cubes * max_blue_cubes;
    }

    Some(power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
