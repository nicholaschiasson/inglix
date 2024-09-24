use std::io;

use inglix::{Word, word::WordError};

use crate::{
	dictionary::Dictionary,
	transpile::Transpile,
};

pub struct InteractiveTranspiler {
	dictionary: Box<dyn Dictionary>,
}

impl InteractiveTranspiler {
	pub fn new(dictionary: Box<dyn Dictionary>) -> Self {
		Self { dictionary }
	}
}

impl Transpile for InteractiveTranspiler {
	fn transpile(&mut self, word: &str) -> Result<Word, String> {
		let stdin = io::stdin();

		if let Some(w) = self.dictionary.lookup(word) {
			Ok(w)
		} else {
			eprint!("No transpilation for '{word}': ");
			let mut transpiled_word = String::from(" ");
			while let Err(WordError::Any(_)) = Word::try_from(&transpiled_word) {
				transpiled_word.clear();
				stdin
					.read_line(&mut transpiled_word)
					.expect("Read user input for yet untranspiled word");
				transpiled_word = transpiled_word.trim().to_string();
				match ((&transpiled_word).try_into(), transpiled_word.is_empty()) {
					(Ok(tw), false) => {
						self
							.dictionary
							.upsert(word, &tw)
							.expect("Update dictionary with new transpilation");
					},
					(Err(WordError::Any(e)), false) => {
						eprintln!("Bad input: {}", e);
						eprint!("Try again: ");
					},
					_ => (),
				}
			}
			if transpiled_word.is_empty() {
				Err(format!(
					"No transpilation found or provided for word '{}'",
					word
				))
			} else {
				transpiled_word
					.try_into()
					.map_err(|e: WordError| e.to_string())
			}
		}
	}
}
