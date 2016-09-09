//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::io;
use stream::entry::header;
use super::Header;

bitflags! {
	pub flags Status: u8 {
		const SEEN     = 0b00000001,
		const OLD      = 0b00000010,
		const ANSWERED = 0b00000100,
		const FLAGGED  = 0b00001000,
		const DRAFT    = 0b00010000,
		const DELETED  = 0b00100000,
	}
}

impl Header for Status {
	#[inline]
	fn name() -> &'static str {
		"Status"
	}

	#[inline]
	fn parse(values: &[header::Item]) -> io::Result<Self> {
		let mut status = Status::empty();

		for ch in values[0].chars() {
			status |= match ch {
				'R' => SEEN,
				'O' => OLD,
				'A' => ANSWERED,
				'F' => FLAGGED,
				'T' => DRAFT,
				'D' => DELETED,

				_ =>
					return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid status"))
			}
		}

		Ok(status)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn read() {
		assert_eq!(<Status as Header>::from_str("R").unwrap(), SEEN);
	}

	#[test]
	fn old() {
		assert_eq!(Status::from_str("O").unwrap(), OLD);
	}

	#[test]
	fn answered() {
		assert_eq!(Status::from_str("A").unwrap(), ANSWERED);
	}

	#[test]
	fn flagged() {
		assert_eq!(Status::from_str("F").unwrap(), FLAGGED);
	}

	#[test]
	fn draft() {
		assert_eq!(Status::from_str("T").unwrap(), DRAFT);
	}

	#[test]
	fn deleted() {
		assert_eq!(Status::from_str("D").unwrap(), DELETED);
	}

	#[test]
	fn mixed() {
		assert_eq!(Status::from_str("ROD").unwrap(), SEEN | OLD | DELETED);
		assert_eq!(Status::from_str("FTA").unwrap(), FLAGGED | DRAFT | ANSWERED);
	}

	#[test]
	fn fail() {
		assert!(Status::from_str("ANTANI").is_err());
	}
}
