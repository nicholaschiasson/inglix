use std::fmt::Display;

use sqlx::SqlitePool;
use uuid::fmt::Hyphenated as Uuid;

use crate::model::Word;

#[derive(Clone, Debug)]
pub enum SearchWord {
	English(String),
	Inglix(inglix::Word),
}

impl SearchWord {
	fn field(&self) -> &str {
		match self {
			SearchWord::English(_) => "english_spelling",
			SearchWord::Inglix(_) => "inglix_spelling",
		}
	}
}

impl Display for SearchWord {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SearchWord::English(s) => write!(f, "{s}"),
			SearchWord::Inglix(s) => write!(f, "{s}"),
		}
	}
}

pub async fn create_word(db: &SqlitePool, word: &Word) -> Result<Uuid, sqlx::Error> {
	sqlx::query("INSERT INTO word VALUES (?, ?, ?)")
		.bind(word.id)
		.bind(&word.english_spelling)
		.bind(&word.inglix_spelling)
		.execute(db)
		.await
		.map(|_| word.id)
}

pub async fn delete_word_by_id(db: &SqlitePool, id: Uuid) -> Result<Uuid, sqlx::Error> {
	sqlx::query("DELETE FROM word WHERE id = ?")
		.bind(id)
		.execute(db)
		.await
		.map(|_| id)
}

pub async fn get_word_by_id(db: &SqlitePool, id: Uuid) -> Result<Word, sqlx::Error> {
	sqlx::query_as::<_, Word>("SELECT * FROM word WHERE id = ?")
		.bind(id)
		.fetch_one(db)
		.await
}

pub async fn get_words(
	db: &SqlitePool,
	search: Option<SearchWord>,
) -> Result<Vec<Word>, sqlx::Error> {
	let search_field = search
		.as_ref()
		.map(|s| format!("WHERE {} LIKE ?", s.field()))
		.unwrap_or_default();
	let search_value = search
		.as_ref()
		.map(|s| format!("%{}%", s))
		.unwrap_or_default();
	sqlx::query_as::<_, Word>(&format!("SELECT * FROM word {}", search_field))
		.bind(search_value)
		.fetch_all(db)
		.await
}
