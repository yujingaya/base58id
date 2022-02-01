//! This crate provides [`Base58id`], a short, human-readable identifier for
//! infrequently generated records.
//!
//! * **Short.** 11 characters long.
//! * **Human-Readable.** No similar looking characters like 0 (zero) and O (capital o).
//! * **URL-Friendly.** Alphanumeric characters only.
//! * **Efficient.** Nicely fits into `u64`.
//!
//! It looks like `3mJr7AoUXx2`.
use std::array::TryFromSliceError;
use std::fmt;
use std::str::{self, FromStr};

use tables::*;

mod tables;

/// A short, human-readable identifier for infrequently generated records
///
/// `Base58id` is essentially a [`u64`] where its textural representation is
/// itself encoded in [Base58](https://tools.ietf.org/id/draft-msporny-base58-01.html).
///
/// ```
/// # use std::collections::HashMap;
/// # use std::str;
/// use base58id::Base58id;
///
/// let encoded = "3mJr7AoUXx2";
///
/// // Parse string
/// let id: Base58id = encoded.parse().unwrap();
///
/// // Make it back to string
/// let id_string: String = id.to_string();
/// assert_eq!(id_string, encoded);
///
/// // Or &str
/// let id_bytes: [u8; 11] = id.into();
/// let id_str = str::from_utf8(&id_bytes).unwrap();
/// assert_eq!(id_str, encoded);
/// 
/// // Use it as a key
/// let mut friends: HashMap<Base58id, String> = HashMap::new();
/// friends.insert(id, "Paul".to_string());
/// ```
///
/// # Entropy
///
/// |                           | n               | log<sub>2</sub>(n) | Example                                |
/// | ------------------------- | --------------- | ------------------ | -------------------------------------- |
/// | Base58id                  | 2<sup>64</sup>  | 64                 | `3mJr7AoUXx2`                          |
/// | YouTube Video Identifiers | 64<sup>11</sup> | 66                 | `lJ3NC-R3gSI`                          |
/// | UUIDv4                    | 2<sup>122</sup> | 122                | `550e8400-e29b-41d4-a716-446655440000` |
///
/// Unlike [UUID], `Base58id` does not provide enough entropy to generate
/// practically unique identifiers. If a user makes 1,000,000 records identified
/// with `Base58id` every day for 10 years, there's about [30.3% chance] to have
/// at least one collision. Therefore, it is advisable to have some collision
/// avoidance mechanism, or use it only when you're sure it's generated only
/// once in a while.
///
/// [UUID]: http://en.wikipedia.org/wiki/Universally_unique_identifier
/// [30.3% chance]: https://www.wolframalpha.com/input/?i=1-binom%282%5E64%2C+3650000000%29*3650000000%21%2F%282%5E%2864*3650000000%29%29
///
/// `Base58id` provides entropy similar to what YouTube is using for their video
/// identifier. They are both 11 digit, and mostly alphanumeric numbers. Hence,
/// you can consider using `Base58id` if you're dealing with something that will
/// be generated less frequently than YouTube videos.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Base58id(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParseBase58idError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TryFromBytesError;

impl Base58id {
    /// Creates a `Base58id`
    pub const fn new(n: u64) -> Self {
        Self(n)
    }

    /// Returns the value as a `u64`.
    pub const fn get(&self) -> u64 {
        self.0
    }
}

impl Base58id {
    const RADIX: u64 = 58;
    const LENGTH: usize = 11;

    // Creates a `Base58id` using the supplied digits
    fn with_digits(digits: [u8; Self::LENGTH]) -> Option<Self> {
        let max = Base58id(u64::MAX).to_digits();

        if digits.iter().any(|&digit| digit >= Self::RADIX as u8) || digits > max {
            return None;
        }

        let n = digits
            .iter()
            .fold(0, |n, &digit| n * Self::RADIX + digit as u64);

        Some(Self(n))
    }

    // Converts a `Base58id` into an array of digits.
    fn to_digits(&self) -> [u8; Self::LENGTH] {
        let mut n = self.0;
        let mut digits = [0u8; Self::LENGTH];

        for digit in digits.iter_mut().rev() {
            *digit = (n % Self::RADIX) as u8;
            n /= Self::RADIX;
        }

        digits
    }
}

impl fmt::Display for Base58id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buf: [u8; Self::LENGTH] = (*self).into();
        let str = str::from_utf8(&buf).unwrap();
        f.write_str(str)
    }
}

impl fmt::Debug for Base58id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buf: [u8; Self::LENGTH] = (*self).into();
        let str = str::from_utf8(&buf).unwrap();
        f.debug_tuple("Base58id").field(&str).finish()
    }
}

impl FromStr for Base58id {
    type Err = ParseBase58idError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; Self::LENGTH] = s.as_bytes().try_into()?;
        Ok(bytes.try_into()?)
    }
}

impl From<Base58id> for [u8; Base58id::LENGTH] {
    fn from(base58id: Base58id) -> Self {
        base58id.to_digits().map(|digit| ENCODE[digit as usize])
    }
}

impl TryFrom<[u8; Self::LENGTH]> for Base58id {
    type Error = TryFromBytesError;

    fn try_from(bytes: [u8; Self::LENGTH]) -> Result<Self, Self::Error> {
        let digits = bytes.map(|byte| DECODE[byte as usize]);
        Self::with_digits(digits).ok_or(TryFromBytesError)
    }
}

impl From<u64> for Base58id {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<Base58id> for u64 {
    fn from(base58id: Base58id) -> Self {
        base58id.0
    }
}

impl From<TryFromSliceError> for ParseBase58idError {
    fn from(_: TryFromSliceError) -> Self {
        ParseBase58idError
    }
}

impl From<TryFromBytesError> for ParseBase58idError {
    fn from(_: TryFromBytesError) -> Self {
        ParseBase58idError
    }
}

#[cfg(test)]
mod tests {
    use crate::{Base58id, ParseBase58idError};

    const TEST_CASES: &[(&str, Result<Base58id, ParseBase58idError>)] = &[
        // Basic
        ("3mJr7AoUXx2", Ok(Base58id(1190710896801901927))),
        ("11111233QC4", Ok(Base58id(0x0000287fb4cd))),

        // Boundaries
        ("11111111111", Ok(Base58id(u64::MIN))),
        ("jpXCZedGfVQ", Ok(Base58id(u64::MAX))),
        ("jpXCZedGfVR", Err(ParseBase58idError)),

        // Wrong input
        ("", Err(ParseBase58idError)), // Too short
        ("3mJr7AoUXx", Err(ParseBase58idError)), // Too short
        ("jpXCZedGfVQ1", Err(ParseBase58idError)), // Too long
        ("zzzzzzzzzzz", Err(ParseBase58idError)), // Out of u64 range
    ];

    #[test]
    fn displays_correctly() {
        for (encoded, result) in TEST_CASES {
            if let Ok(id) = result {
                assert_eq!(encoded, &id.to_string());
            }
        }
    }

    #[test]
    fn parses_strings_correctly() {
        for (encoded, result) in TEST_CASES {
            assert_eq!(encoded.parse::<Base58id>(), *result);
        }
    }
}
