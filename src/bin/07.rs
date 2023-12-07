use std::cmp::Ordering;
use itertools::Itertools;

advent_of_code::solution!(7);

const CARD_VALUES: &'static str = "23456789TJQKA";
const HANDS: &'static str = "H123F45";

fn sort_card(card: &str) -> String {
    let mut card_split = card.chars().collect::<Vec<char>>().iter().map(|c| c.to_string()).collect::<Vec<String>>();

    card_split.sort_by(|a,b| {
        CARD_VALUES.find(a).unwrap().cmp(
            &CARD_VALUES.find(b).unwrap()
        )
    });

    card_split.join("")
}

fn occurences_map(cards: &str) -> usize {
    let groups = cards
        .chars()
        .into_grouping_map_by(|&x| x)
        .fold(0, |acc, _key, _value| acc + 1);

    let values = groups.values().collect::<Vec<&i32>>().iter().sorted().join("");

    if values.contains("5") {
        return HANDS.find("5").unwrap();
    }

    if values.contains("4") {
        return HANDS.find("4").unwrap();
    }

    if values.contains("3") && values.contains("2") {
        return HANDS.find("F").unwrap();
    }

    if values.contains("3") {
        return HANDS.find("3").unwrap();
    }

    if values.contains("22") {
        return HANDS.find("2").unwrap();
    }

    if values.contains("2") {
        return HANDS.find("1").unwrap();
    }

    if CARD_VALUES.contains(&sort_card(&cards)) {
        return HANDS.find("H").unwrap();
    }

    0
}

fn sort_card_lines(a: &Vec<&str>, b: &Vec<&str>) -> Ordering {
    let a_cards = a[0];
    let b_cards = b[0];

    let a_card_occurences = occurences_map(a_cards);
    let b_card_occurences = occurences_map(b_cards);

    if a_card_occurences > b_card_occurences {
        return Ordering::Greater;
    } else if a_card_occurences < b_card_occurences {
        return Ordering::Less;
    }

    // Order by card value
    for card_index in 0..a_cards.len() {
        let a_card: &str = &a_cards[card_index..card_index + 1];
        let b_card: &str = &b_cards[card_index..card_index + 1];

        let a_card_pos = CARD_VALUES.find(a_card).unwrap();
        let b_card_pos = CARD_VALUES.find(b_card).unwrap();

        if a_card_pos > b_card_pos {
            return Ordering::Greater;
        } else if a_card_pos < b_card_pos {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut card_lines: Vec<Vec<&str>> = lines.iter().map(|line| line.split(' ').collect()).collect::<Vec<Vec<&str>>>();

    card_lines.sort_by(sort_card_lines);

    let results: Vec<u32> = card_lines.iter().map(|l| l[1].parse::<u32>().unwrap()).collect();
    let mut winnings: u32 = 0;

    for (index, result) in results.iter().enumerate() {
        winnings += result * (index + 1) as u32;
    }

    Some(winnings)
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
