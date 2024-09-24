// @generated automatically by Diesel CLI.

diesel::table! {
	words (id) {
		id -> Uuid,
		english_spelling -> Varchar,
		inglix_spelling -> Varchar,
	}
}
