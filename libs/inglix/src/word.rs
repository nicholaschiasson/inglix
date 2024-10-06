use std::{error::Error, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlite")]
use sqlx::{Decode, Encode, Type};

use crate::grapheme::Grapheme;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WordError {
	Any(String),
	ZeroLength,
}

impl Display for WordError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl Error for WordError {}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(into = "String")]
#[serde(try_from = "&str")]
pub struct Word {
	initial: Grapheme,
	rest: Vec<Grapheme>,
	homonym_index: usize,
}

impl Word {
	pub fn new(initial: Grapheme, rest: &[Grapheme]) -> Self {
		let len = rest.len();
		let rest = rest
			.iter()
			.skip_while(|&grapheme| initial.eq(grapheme))
			.copied()
			.collect::<Vec<_>>();
		let homonym_index = len - rest.len();
		Self {
			initial,
			rest,
			homonym_index,
		}
	}

	pub fn make_homonym(&mut self) {
		self.homonym_index += 1;
	}
}

impl Display for Word {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			[self.initial]
				.iter()
				.cycle()
				.take(self.homonym_index + 1)
				.chain(self.rest.iter())
				.map(|&grapheme| char::from(grapheme))
				.collect::<String>()
		)
	}
}

impl FromStr for Word {
	type Err = WordError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let graphemes = s
			.chars()
			.map(Grapheme::try_from)
			.collect::<Result<Vec<_>, _>>()
			.map_err(WordError::Any)?;
		match graphemes.as_slice() {
			[initial, rest @ ..] => Ok(Self::new(*initial, rest)),
			[] => Err(WordError::ZeroLength),
		}
	}
}

impl TryFrom<&str> for Word {
	type Error = WordError;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		s.parse::<Self>()
	}
}

impl TryFrom<String> for Word {
	type Error = WordError;

	fn try_from(s: String) -> Result<Self, Self::Error> {
		s.parse::<Self>()
	}
}

impl TryFrom<&String> for Word {
	type Error = WordError;

	fn try_from(s: &String) -> Result<Self, Self::Error> {
		s.parse::<Self>()
	}
}

impl From<Word> for String {
	fn from(w: Word) -> Self {
		w.to_string()
	}
}

#[cfg(feature = "sqlite")]
impl<'r> Decode<'r, sqlx::Sqlite> for Word {
	fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
		let str_value = <&str as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
		str_value.parse().map_err(sqlx::error::BoxDynError::from)
	}
}

#[cfg(feature = "sqlite")]
impl<'q> Encode<'q, sqlx::Sqlite> for Word {
	fn encode_by_ref(
		&self,
		buf: &mut <sqlx::Sqlite as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
	) -> sqlx::encode::IsNull {
		<String as Encode<sqlx::Sqlite>>::encode(self.to_string(), buf)
	}
}

#[cfg(feature = "sqlite")]
impl Type<sqlx::Sqlite> for Word {
	fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
		<String as Type<sqlx::Sqlite>>::type_info()
	}
}

#[cfg(test)]
mod test {
	use crate::Word;

	#[test]
	fn homonyms() {
		use crate::Grapheme::*;

		let word = Word::new(t, &[t, t, U]);
		assert_eq!(word.initial, t);
		assert_eq!(word.rest, &[U]);
		assert_eq!(word.homonym_index, 2);
	}

	#[test]
	fn into_and_from_string() {
		assert_eq!(
			Ok("dHblyU".to_string()),
			"dHblyU".parse::<Word>().map(|w| w.to_string())
		);
	}
}
