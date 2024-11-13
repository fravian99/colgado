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

    pub fn check_word(&mut self, word: &str) -> usize {
        let mut num = 0;

        word.graphemes(true).for_each(|word_char| {
            if self.tried.contains(&word_char.into()) {
                return;
            }
            for (i, character) in self.characters.iter().enumerate() {
                if self.progress[i] && character.eq_ignore_ascii_case(word_char) {
                    break;
                } else if !self.progress[i] && character.eq_ignore_ascii_case(word_char) {
                    self.progress[i] = true;
                    self.cont -= 1;
                    num += 1;
                } else {
                    match self.tried.last() {
                        Some(tried) if tried.deref() == word_char => {}
                        _ => {
                            if !self.characters.contains(&word_char.into()) {
                                self.tried.push(word_char.into());
                            }
                        }
                    }
                }
            }
        });

        num
    }

    pub fn get_actual_word(&self) -> String {
        let mut string = String::with_capacity(self.characters.len());
        for (i, letter) in self.characters.iter().enumerate() {
            let show = self.progress.get(i);
            if let Some(true) = show {
                string += letter;
            } else {
                string += "_";
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
        let non_ascii_word_string = "camión".to_string().to_ascii_lowercase();
        let mut word = Game::new(non_ascii_word_string.clone());
        println!("{:?}", word.progress);
        assert_eq!(word.check_word(&non_ascii_word_string), 6);
        println!("{:?}", word.progress);

        let non_ascii_word_string = "camión".to_string().to_ascii_lowercase();
        let mut word = Game::new(non_ascii_word_string.clone());
        println!("{:?}", word.progress);
        assert_eq!(word.check_word("camió"), 5);
        println!("{:?}", word.progress);

        let ascii_word = "camion".to_string();
        let mut word = Game::new(non_ascii_word_string.clone());
        assert_eq!(word.check_word(&ascii_word), 5);
    }

    #[test]
    fn check_correct_letters() {
        let mut word = Game::new("prueba".to_owned());
        word.check_word("prueba");
        assert_eq!(word.get_letters(), "");
    }

    #[test]
    fn check_letters_() {
        let mut word = Game::new("prueba".to_owned());
        word.check_word("oighqs");
        let solution = "o i g h q s ";
        assert_eq!(word.get_letters(), solution);
    }

    #[test]
    fn with_whitespaces() {
        let input = "prueba con espacios";
        let tried = "pruebaconespacios";

        let mut word = Game::new(input.to_owned());

        assert_eq!(word.check_word(tried), tried.len());

        println!("{:?}", word);
        assert!(word.get_letters().is_empty());
        assert!(word.is_completed());
    }
}
