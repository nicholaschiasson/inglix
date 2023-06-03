use inglix::word;
use uuid::Uuid;

use crate::schema::words;

#[derive(Insertable)]
#[table_name = "words"]
pub struct NewWord<'a> {
	pub english_spelling: &'a str,
	pub inglix_spelling: &'a str,
}

// #[derive(Debug, Queryable, AsChangeset)]
// pub struct Word {
// 	pub id: Uuid,
// 	english_spelling: String,
// 	inglix_spelling: Word,
// }
