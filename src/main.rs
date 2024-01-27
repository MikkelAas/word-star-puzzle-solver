use std::collections::HashSet;

struct PuzzleInput {
    must_include_letter: char,
    allowed_characters: Vec<char>,
}

fn main() {
    let puzzle_input = PuzzleInput {
        must_include_letter: 'P',
        allowed_characters: vec!['B', 'L', 'Å', 'Æ', 'O', 'E', 'P'],
    };

    let lines = read_file_to_lines();

    let result = filter_puzzle_input(lines, puzzle_input);

    println!("{:?}", result)
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
                .contains(puzzle_input.must_include_letter)
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
            must_include_letter: 'U',
            allowed_characters: vec!['T', 'U', 'R'],
        };

        let result = filter_puzzle_input(words, puzzle_input);

        let expected = HashSet::from([String::from("Tur")]);

        assert_eq!(expected, result)
    }
}
