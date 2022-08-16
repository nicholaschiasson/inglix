use crate::word::Word;

pub trait Dictionary {
	fn lookup(&self, word: &String) -> Option<Word>;
	fn upsert(&mut self, word: &String, traspilation: &Word) -> Result<(), String>;
}
