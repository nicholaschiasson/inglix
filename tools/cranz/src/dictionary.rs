use inglix::Word;

pub trait Dictionary {
	fn lookup(&self, word: &str) -> Option<Word>;
	fn upsert(&mut self, word: &str, traspilation: &Word) -> Result<(), String>;
}
