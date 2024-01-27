pub struct PuzzleInput {
    pub required_character: char,
    pub allowed_characters: Vec<char>,
}

impl PuzzleInput {
    pub fn new(required_character: char, allowed_characters: Vec<char>) -> Self {
        if allowed_characters.len() != 7 {
            panic!("List length of allowed characters must be exactly 7")
        }        

        Self {
            required_character,
            allowed_characters,
        }
    }
}
