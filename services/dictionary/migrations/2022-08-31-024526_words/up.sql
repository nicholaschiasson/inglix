CREATE TABLE words (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	english_spelling VARCHAR(255) UNIQUE NOT NULL,
	inglix_spelling VARCHAR(255) UNIQUE NOT NULL
);
