#![forbid(unsafe_code)]

use std::{
	error::Error,
	fs::File,
	io::{BufRead, BufReader},
	path::PathBuf,
};

use clap::Parser;
use interactive_transpiler::InteractiveTranspiler;
use local_dictionary::LocalDictionary;

use crate::transpile::Transpile;

mod dictionary;
mod interactive_transpiler;
mod local_dictionary;
mod serializer;
mod transpile;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	/// File to transpile from English into inglix
	#[clap(value_parser)]
	file: PathBuf,
	/// File to use as persistant dictionary
	#[clap(
		short,
		long,
		default_value = "./English_inglix_Dictionary.toml",
		value_parser
	)]
	dictionary: String,
}

/**
 * What this program does:
 * 1. Read file line by line.
 * 2. Buffer non-word characters to transpiled line.
 * 3. Perform word lookups in English to inglix dictionary.
 * 4. Ask user for word transpilation if word not yet in dictionary.
 * 5. Add newly transpiled word to dictionary.
 * 6. Buffer transpiled word to transpiled line.
 * 7. Write transpiled line to standard out.
 *
 * To Do:
 * - '=' handler
 * - apostrophe hanlder
 * - hyphen handler
 * - acronym handler
 * - proper noun handler
 * - dictionary entry metadata/additional fields
 * - bi-directional dictionary lookups
 * - bi-directional transpilation
 * - command line arguments to determine manual or automatic transpilation
 * - command line arguments to determine local or cloud dictionary
 * - command line arguments for credentials for cloud dictionary
 */
fn main() -> Result<(), Box<dyn Error>> {
	let cli = Cli::parse();
	let f = File::open(cli.file)?;
	let reader = BufReader::new(f);

	let mut transpiler = InteractiveTranspiler::new(Box::new(LocalDictionary::new(
		&PathBuf::try_from(cli.dictionary).expect("Valid path to dictionary file"),
	)));

	for line in reader.lines() {
		let line = line.expect("Read line from file");
		let mut out_line = String::new();
		let mut word = String::new();
		for c in line.chars() {
			if !c.is_alphabetic() && c != '\'' {
				if !word.is_empty() {
					// end of word, dump word
					if let Ok(w) = transpiler.transpile(&word) {
						out_line.push_str(&w.to_string());
					} else {
						out_line.push_str(&word);
					}
					word.clear();
				}
				out_line.push(c);
			} else {
				// word character, append to current word
				word.push(c);
			}
		}
		println!("{out_line}");
	}

	Ok(())
}
