use std::collections::HashSet;

use crate::model::PuzzleInput;

pub fn solve_word_star_puzzle(puzzle_input: PuzzleInput) -> HashSet<String> {
    let words = read_file_to_lines();

    filter_puzzle_input(words, puzzle_input)
}

fn read_file_to_lines() -> Vec<String> {
    include_str!("words.csv")
        .to_owned()
        .lines()
        .map(String::from)
        .collect()
}

fn filter_puzzle_input(lines: Vec<String>, puzzle_input: PuzzleInput) -> HashSet<String> {
    let filtered_lines: Vec<String> = lines
        .into_iter()
        .filter(|line| line.len() > 3)
        .filter(|line| {
            line.to_ascii_uppercase()
                .contains(puzzle_input.required_character)
        })
        .filter(|line| {
            line.chars().all(|char| {
                puzzle_input
                    .allowed_characters
                    .contains(&char.to_ascii_uppercase())
            })
        })
        .collect();

    HashSet::from_iter(filtered_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_puzzle_input() {
        let words = vec![
            String::from("Tennis"),
            String::from("Videospill"),
            String::from("Tur"),
            String::from("Kaffe"),
        ];

        let puzzle_input = PuzzleInput {
            required_character: 'U',
            allowed_characters: vec!['T', 'U', 'R'],
        };

        let result = filter_puzzle_input(words, puzzle_input);

        let expected = HashSet::from([String::from("Tur")]);

        assert_eq!(expected, result)
    }
}
