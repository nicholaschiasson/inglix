use std::ffi::OsStr;

use serde::{Deserialize, Serialize};

pub enum Serializer {
	Json,
	Toml,
	Yaml,
}

impl Serializer {
	pub fn empty_file(&self) -> String {
		match self {
			&Serializer::Json => String::from("{}"),
			_ => String::from(""),
		}
	}

	pub fn from_str<'a, T: Deserialize<'a>>(&self, s: &'a str) -> Result<T, String> {
		match self {
			Serializer::Json => serde_json::from_str(s).map_err(|e| format!("{e}")),
			Serializer::Toml => toml::from_str(s).map_err(|e| format!("{e}")),
			Serializer::Yaml => serde_yaml::from_str(s).map_err(|e| format!("{e}")),
		}
	}

	pub fn to_string<T: Serialize>(&self, value: &T) -> Result<String, String> {
		match self {
			Serializer::Json => serde_json::to_string(value).map_err(|e| format!("{e}")),
			Serializer::Toml => toml::to_string(value).map_err(|e| format!("{e}")),
			Serializer::Yaml => serde_yaml::to_string(value).map_err(|e| format!("{e}")),
		}
	}
}

impl TryFrom<&OsStr> for Serializer {
	type Error = String;

	fn try_from(s: &OsStr) -> Result<Self, Self::Error> {
		match s.to_ascii_lowercase().to_str() {
			Some("json") => Ok(Self::Json),
			Some("toml") => Ok(Self::Toml),
			Some("yaml") => Ok(Self::Yaml),
			Some(t) => Err(format!("Invalid file extension '{}'", t)),
			None => Err(String::from("Invalid file without extension")),
		}
	}
}
