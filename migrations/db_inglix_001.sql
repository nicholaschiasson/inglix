DROP TABLE IF EXISTS word;

CREATE TABLE word (
	id UUID NOT NULL,
	english_spelling TEXT NOT NULL,
	inglix_spelling TEXT NOT NULL,
	PRIMARY KEY (`id`),
	UNIQUE(`english_spelling`, `inglix_spelling`)
);
