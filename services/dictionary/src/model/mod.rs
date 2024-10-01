use serde::Serialize;
use sqlx::FromRow;
use uuid::fmt::Hyphenated as Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Word {
	pub id: Uuid,
	pub english_spelling: String,
	pub inglix_spelling: String,
}

impl Word {
	pub fn new(english_spelling: &str, inglix_spelling: &str) -> Self {
		Self {
			id: uuid::Uuid::new_v4().into(),
			english_spelling: english_spelling.into(),
			inglix_spelling: inglix_spelling.into(),
		}
	}
}
