use crate::word::Game;

#[derive(Debug, Clone)]
pub struct GameView {
    pub word: String,
    pub letters: String,
    pub is_completed: bool,
}

impl GameView {
    pub fn new(word: String, letters: String, is_completed: bool) -> Self {
        Self {
            word,
            letters,
            is_completed,
        }
    }
}

impl From<&Game> for GameView {
    fn from(value: &Game) -> Self {
        Self {
            word: value.get_actual_word(),
            letters: value.get_letters(),
            is_completed: value.is_completed(),
        }
    }
}

impl Default for GameView {
    fn default() -> Self {
        Self {
            word: String::default(),
            letters: String::default(),
            is_completed: true,
        }
    }
}
