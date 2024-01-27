pub struct PuzzleInput {
    pub required_character: char,
    pub allowed_characters: Vec<char>,
}

impl PuzzleInput {
    pub fn new(required_character: char, allowed_characters: Vec<char>) -> Self {
        if allowed_characters.len() != 7 {
            panic!("List length of allowed characters must be exactly 7")
        }

        let required_character_uppercase = required_character.to_ascii_lowercase();

        let allowed_characters_uppercase: Vec<char> = allowed_characters
            .clone()
            .into_iter()
            .map(|char| char.to_ascii_lowercase())
            .collect();

        Self {
            required_character: required_character_uppercase,
            allowed_characters: allowed_characters_uppercase,
        }
    }
}
