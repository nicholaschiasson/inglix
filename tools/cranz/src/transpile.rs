use inglix::Word;

pub trait Transpile {
	fn transpile(&mut self, word: &str) -> Result<Word, String>;
}
