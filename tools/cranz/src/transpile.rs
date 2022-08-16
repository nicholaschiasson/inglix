use crate::word::Word;

pub trait Transpile {
	fn transpile(&mut self, word: &String) -> Result<Word, String>;
}
