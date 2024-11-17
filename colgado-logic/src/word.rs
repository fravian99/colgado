use crate::errors::GameError;
use std::ops::Deref;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub struct Game {
    characters: Vec<String>,
    progress: Vec<bool>,
    tried: Vec<String>,
    cont: usize,
}
impl Game {
    pub fn new(mut word: String) -> Self {
        word.retain(|character| !character.is_whitespace());
        let characters: Vec<String> = word
            .graphemes(true)
            .map(|string| string.to_owned())
            .collect();
        let progress: Vec<bool> = vec![false; characters.len()];
        let cont = characters.len();

        Self {
            characters,
            progress,
            tried: Vec::new(),
            cont,
        }
    }

    pub fn check_word(&mut self, word: &str) -> Result<usize, GameError> {
        let mut num = 0;
        let word_chars: Vec<&str> = word.graphemes(true).collect();
        if word_chars.len() > self.characters.len() {
            return Err(GameError::InvalidWord);
        }
        word_chars.iter().for_each(|word_char| {
            let word_char = *word_char;
            if self.tried.contains(&word_char.into()) {
                return;
            }
            for (i, character) in self.characters.iter().enumerate() {
                let character: &str = character;
                if self.progress[i] && character == word_char {
                    break;
                }
                if character == word_char {
                    self.progress[i] = true;
                    self.cont -= 1;
                    num += 1;
                } else {
                    match self.tried.last() {
                        Some(tried) if tried.deref() == word_char => {}
                        _ => {
                            let word_char = word_char.into();
                            if !self.characters.contains(&word_char) {
                                self.tried.push(word_char);
                            }
                        }
                    }
                }
            }
        });

        Ok(num)
    }

    pub fn get_actual_word(&self) -> String {
        let mut string = String::with_capacity(self.characters.len());
        for (i, letter) in self.characters.iter().enumerate() {
            match self.progress.get(i) {
                Some(true) => string += letter,
                _ => string += "_",
            }
        }
        string
    }

    pub fn get_letters(&self) -> String {
        let mut string = String::with_capacity(self.characters.len());
        self.tried.iter().for_each(|letter| {
            string += letter;
            string += " ";
        });
        string
    }
    pub fn is_completed(&self) -> bool {
        self.cont == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_word_with_non_ascii_char() {
        let non_ascii_word_string = "camión".to_owned().to_ascii_lowercase();
        let mut word = Game::new(non_ascii_word_string.clone());
        assert_eq!(word.check_word(&non_ascii_word_string).unwrap(), 6);

        let non_ascii_word_string = "camión".to_owned().to_ascii_lowercase();
        let mut word = Game::new(non_ascii_word_string.clone());
        assert_eq!(word.check_word("camió").unwrap(), 5);

        let ascii_word = "camion".to_owned();
        let mut word = Game::new(non_ascii_word_string.clone());
        assert_eq!(word.check_word(&ascii_word).unwrap(), 5);

        let non_ascii_word_string = "aab".to_owned().to_ascii_lowercase();
        let mut word = Game::new(non_ascii_word_string.clone());
        assert_eq!(word.check_word("ab").unwrap(), 3);
    }

    #[test]
    fn check_word_with_uppercase() {
        let mut word = Game::new("Prueba".to_owned());
        word.check_word("prueba").unwrap();
        assert_eq!(word.get_letters(), "p ");
    }

    #[test]
    fn check_correct_letters() {
        let mut word = Game::new("prueba".to_owned());
        word.check_word("prueba").unwrap();
        assert_eq!(word.get_letters(), "");
    }

    #[test]
    fn check_letters_() {
        let mut word = Game::new("prueba".to_owned());
        word.check_word("oighqs").unwrap();
        let solution = "o i g h q s ";
        assert_eq!(word.get_letters(), solution);
    }

    #[test]
    fn with_whitespaces() {
        let input = "prueba con espacios";
        let tried = "pruebaconespacios";

        let mut word = Game::new(input.to_owned());

        assert_eq!(word.check_word(tried).unwrap(), tried.len());

        println!("{:?}", word);
        assert!(word.get_letters().is_empty());
        assert!(word.is_completed());
    }
}
