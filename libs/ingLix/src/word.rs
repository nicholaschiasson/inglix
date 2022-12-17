use std::fmt;

// use diesel::{backend::Backend, deserialize::{FromSql, self}, types::Varchar};
use serde::{Deserialize, Serialize};

use crate::grapheme::Grapheme;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WordError {
	Any(String),
	ZeroLength,
}

impl fmt::Display for WordError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Any(e) => e,
				Self::ZeroLength => "empty string is not a valid word",
			}
		)
	}
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(into = "String")]
#[serde(try_from = "String")]
pub struct Word {
	characters: Vec<Grapheme>,
	homonym_index: usize,
}

impl Word {
	pub fn make_homonym(&mut self) {
		self.homonym_index += 1;
	}
}

impl From<Word> for String {
	fn from(w: Word) -> Self {
		w.to_string()
	}
}

// impl<DB> FromSql<Varchar, DB> for Word
// where
// 	DB: Backend
// {
// 	fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
// 		Ok("a".to_string().try_into().unwrap())
// 	}
// }

impl TryFrom<String> for Word {
	type Error = WordError;

	fn try_from(s: String) -> Result<Self, Self::Error> {
		Self::try_from(&s)
	}
}

impl TryFrom<&String> for Word {
	type Error = WordError;

	fn try_from(s: &String) -> Result<Self, Self::Error> {
		let mut characters = Vec::new();
		let (s_trimmed, homonym_index);
		if let Some(c) = s.chars().next() {
			characters.push(c.try_into().map_err(WordError::Any)?);
			s_trimmed = s.trim_start_matches(c);
			homonym_index = s.len() - s_trimmed.len() - 1;
		} else {
			return Err(WordError::ZeroLength);
		}
		for c in s_trimmed.chars() {
			characters.push(c.try_into().map_err(WordError::Any)?);
		}
		Ok(Self {
			characters,
			homonym_index,
		})
	}
}

impl ToString for Word {
	fn to_string(&self) -> String {
		self
			.characters
			.iter()
			.take(1)
			.cycle()
			.take(self.homonym_index)
			.chain(self.characters.iter())
			.map(|&grapheme| char::from(grapheme))
			.collect()
	}
}
