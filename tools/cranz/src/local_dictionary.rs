use std::{
	collections::{HashMap, HashSet},
	fs,
	path::PathBuf,
};

use inglix::Word;
use serde::{Deserialize, Serialize};

use crate::{dictionary::Dictionary, serializer::Serializer};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
#[serde(rename_all = "kebab-case")]
struct EnglishInglixDictionary {
	#[serde(default)]
	English_inglix: HashMap<String, Word>,
	#[serde(default)]
	inglix_English: HashMap<Word, String>,
}

pub struct LocalDictionary {
	file: PathBuf,
	dict: EnglishInglixDictionary,
	serializer: Serializer,
}

impl LocalDictionary {
	pub fn new(file: &PathBuf) -> Self {
		let serializer: Serializer = file
			.extension()
			.unwrap_or_default()
			.try_into()
			.expect("Valid dictionary file path");
		if !file.exists() {
			fs::create_dir_all(file.parent().expect("Get dictionary directory name"))
				.expect("Create dictionary directory");
			fs::write(&file, serializer.empty_file()).expect("Create dictionary file");
		}
		Self {
			file: file.into(),
			dict: serializer
				.from_str(&fs::read_to_string(file).expect("Read dictionary from file"))
				.expect("Deserialize dictionary file"),
			serializer,
		}
	}
}

impl Dictionary for LocalDictionary {
	fn lookup(&self, word: &str) -> Option<Word> {
		self.dict.English_inglix.get(&word.to_lowercase()).cloned()
	}

	fn upsert(&mut self, english_word: &str, inglix_word: &Word) -> Result<(), String> {
		let mut engs = HashSet::from([english_word.to_lowercase()]);
		let mut ing = inglix_word.clone();

		while let Some(eng) = self.dict.inglix_English.get(&ing) {
			engs.insert(eng.to_lowercase());
			ing.make_homonym();
		}

		let mut engs = engs.into_iter().collect::<Vec<_>>();
		engs.sort();
		let mut ing = inglix_word.clone();

		for eng in engs {
			self
				.dict
				.English_inglix
				.insert(eng.to_owned(), ing.to_owned());
			self
				.dict
				.inglix_English
				.insert(ing.to_owned(), eng.to_owned());
			ing.make_homonym()
		}

		fs::write(
			&self.file,
			self
				.serializer
				.to_string(&self.dict)
				.expect("Serialize dictionary"),
		)
		.map_err(|e| e.to_string())
	}
}
