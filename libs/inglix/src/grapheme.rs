use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::Word;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(try_from = "char")]
pub enum Grapheme {
	a,
	b,
	c,
	d,
	e,
	E,
	f,
	g,
	J,
	h,
	i,
	j,
	k,
	l,
	m,
	n,
	o,
	O,
	p,
	r,
	s,
	S,
	t,
	T,
	u,
	U,
	H,
	v,
	w,
	x,
	y,
	z,
}

impl Grapheme {
	pub fn name(&self) -> Word {
		use Grapheme::*;
		match self {
			Self::a => Word::new(e, &[E]),
			Self::b => Word::new(b, &[E]),
			Self::c => Word::new(s, &[E]),
			Self::d => Word::new(d, &[E]),
			Self::e => Word::new(E, &[]),
			Self::E => Word::new(m, &[E]),
			Self::f => Word::new(e, &[f]),
			Self::g => Word::new(j, &[E]),
			Self::J => Word::new(J, &[E]),
			Self::h => Word::new(e, &[E, c]),
			Self::i => Word::new(a, &[E]),
			Self::j => Word::new(j, &[e, E]),
			Self::k => Word::new(k, &[e, E]),
			Self::l => Word::new(e, &[l]),
			Self::m => Word::new(e, &[m]),
			Self::n => Word::new(e, &[n]),
			Self::o => Word::new(O, &[]),
			Self::O => Word::new(n, &[O]),
			Self::p => Word::new(p, &[E]),
			Self::r => Word::new(a, &[r]),
			Self::s => Word::new(e, &[s]),
			Self::S => Word::new(e, &[S]),
			Self::t => Word::new(t, &[E]),
			Self::T => Word::new(T, &[E]),
			Self::u => Word::new(y, &[U]),
			Self::U => Word::new(h, &[U]),
			Self::H => Word::new(h, &[H, m]),
			Self::v => Word::new(v, &[E]),
			Self::w => Word::new(d, &[H, b, l, y, U]),
			Self::x => Word::new(e, &[k, s]),
			Self::y => Word::new(w, &[a, E]),
			Self::z => Word::new(z, &[E]),
		}
	}
}

impl TryFrom<char> for Grapheme {
	type Error = String;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		match c {
			'a' => Ok(Self::a),
			'b' => Ok(Self::b),
			'c' => Ok(Self::c),
			'd' => Ok(Self::d),
			'e' => Ok(Self::e),
			'E' => Ok(Self::E),
			'f' => Ok(Self::f),
			'g' => Ok(Self::g),
			'J' => Ok(Self::J),
			'h' => Ok(Self::h),
			'i' => Ok(Self::i),
			'j' => Ok(Self::j),
			'k' => Ok(Self::k),
			'l' => Ok(Self::l),
			'm' => Ok(Self::m),
			'n' => Ok(Self::n),
			'o' => Ok(Self::o),
			'O' => Ok(Self::O),
			'p' => Ok(Self::p),
			'r' => Ok(Self::r),
			's' => Ok(Self::s),
			'S' => Ok(Self::S),
			't' => Ok(Self::t),
			'T' => Ok(Self::T),
			'u' => Ok(Self::u),
			'U' => Ok(Self::U),
			'H' => Ok(Self::H),
			'v' => Ok(Self::v),
			'w' => Ok(Self::w),
			'x' => Ok(Self::x),
			'y' => Ok(Self::y),
			'z' => Ok(Self::z),
			_ => Err(format!("Invalid grapheme initialization '{}'", c)),
		}
	}
}

impl From<Grapheme> for char {
	fn from(grapheme: Grapheme) -> Self {
		Self::from(&grapheme)
	}
}

impl From<&Grapheme> for char {
	fn from(grapheme: &Grapheme) -> Self {
		match grapheme {
			Grapheme::a => 'a',
			Grapheme::b => 'b',
			Grapheme::c => 'c',
			Grapheme::d => 'd',
			Grapheme::e => 'e',
			Grapheme::E => 'E',
			Grapheme::f => 'f',
			Grapheme::g => 'g',
			Grapheme::J => 'J',
			Grapheme::h => 'h',
			Grapheme::i => 'i',
			Grapheme::j => 'j',
			Grapheme::k => 'k',
			Grapheme::l => 'l',
			Grapheme::m => 'm',
			Grapheme::n => 'n',
			Grapheme::o => 'o',
			Grapheme::O => 'O',
			Grapheme::p => 'p',
			Grapheme::r => 'r',
			Grapheme::s => 's',
			Grapheme::S => 'S',
			Grapheme::t => 't',
			Grapheme::T => 'T',
			Grapheme::u => 'u',
			Grapheme::U => 'U',
			Grapheme::H => 'H',
			Grapheme::v => 'v',
			Grapheme::w => 'w',
			Grapheme::x => 'x',
			Grapheme::y => 'y',
			Grapheme::z => 'z',
		}
	}
}

impl Display for Grapheme {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", char::from(self))
	}
}

impl FromStr for Grapheme {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.chars().collect::<Vec<_>>()[..] {
			[c] => c.try_into(),
			[] => Err("Empty string".to_string()),
			_ => Err(format!("Invalid string '{}'", s)),
		}
	}
}

#[cfg(test)]
mod test {
	use crate::Grapheme;

	#[test]
	fn name() {
		assert_eq!("dHblyU", Grapheme::w.name().to_string());
	}

	#[test]
	fn into_and_from_char() {
		assert_eq!(Ok(Grapheme::a), Grapheme::try_from('a'));
		assert_eq!(
			Err("Invalid grapheme initialization 'q'".to_string()),
			Grapheme::try_from('q')
		);
	}

	#[test]
	fn into_and_from_string() {
		assert_eq!(Ok(Grapheme::a), "a".parse());
		assert_eq!(
			Err("Invalid grapheme initialization 'q'".to_string()),
			"q".parse::<Grapheme>()
		);
		assert_eq!(Err("Empty string".to_string()), "".parse::<Grapheme>());
		assert_eq!(
			Err("Invalid string 'aa'".to_string()),
			"aa".parse::<Grapheme>()
		);
	}
}
