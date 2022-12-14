use lazy_static::lazy_static;
use ibig::IBig;
use num_integer::Integer;
use num_traits::ToPrimitive;
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::{
    internal::coder::{Decoder, Encoder, IntegerBytesCoder},
    Error, Result,
};

use super::Nat;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
}

/// An integer that can be encoded to a Zarith number
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(try_from = "String")
)]
pub struct Int(String);

impl Int {
    pub fn from<S: Into<String>>(value: S) -> Result<Self> {
        let value: String = value.into();
        if Self::is_valid(&value) {
            return Ok(Self(value));
        }
        Err(Error::InvalidIntegerString)
    }

    pub fn from_string(value: String) -> Result<Self> {
        Self::from(value)
    }

    pub fn from_integer<I: Integer + ToString>(value: I) -> Self {
        Self::from_string(value.to_string()).unwrap()
    }

    pub fn to_integer<I: Integer + FromStr>(&self) -> Result<I>
    where
        <I as FromStr>::Err: Debug,
    {
        Ok(self
            .0
            .parse::<I>()
            .map_err(|_error| Error::InvalidIntegerConversion)?)
    }

    pub fn is_valid(value: &str) -> bool {
        REGEX.is_match(value)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        IntegerBytesCoder::encode(self)
    }

    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToPrimitive for Int {
    fn to_i64(&self) -> Option<i64> {
        self.to_integer().ok()
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_integer().ok()
    }

    fn to_i128(&self) -> Option<i128> {
        self.to_integer().ok()
    }

    fn to_u128(&self) -> Option<u128> {
        self.to_integer().ok()
    }
}

impl From<i8> for Int {
    fn from(value: i8) -> Self {
        Self::from_integer(value)
    }
}

impl From<i16> for Int {
    fn from(value: i16) -> Self {
        Self::from_integer(value)
    }
}

impl From<i32> for Int {
    fn from(value: i32) -> Self {
        Self::from_integer(value)
    }
}

impl From<i64> for Int {
    fn from(value: i64) -> Self {
        Self::from_integer(value)
    }
}

impl From<i128> for Int {
    fn from(value: i128) -> Self {
        Self::from_integer(value)
    }
}

impl From<IBig> for Int {
    fn from(value: IBig) -> Self {
        Self::from_string(value.to_string()).unwrap()
    }
}

impl From<Nat> for Int {
    fn from(value: Nat) -> Self {
        Int(value.to_string())
    }
}

impl From<&Nat> for Int {
    fn from(value: &Nat) -> Self {
        Int(value.to_string())
    }
}

impl From<Int> for String {
    fn from(value: Int) -> Self {
        value.0
    }
}

impl TryFrom<String> for Int {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::from_string(value)
    }
}

impl TryFrom<&str> for Int {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_string(value.to_string())
    }
}

impl TryFrom<&Vec<u8>> for Int {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        IntegerBytesCoder::decode(value)
    }
}

impl TryFrom<&Int> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Int) -> Result<Self> {
        value.to_bytes()
    }
}

impl TryFrom<&Int> for IBig {
    type Error = Error;

    fn try_from(value: &Int) -> Result<Self> {
        IBig::from_str_radix(&value.0, 10)
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_integers() -> Result<()> {
        let integer_strings = vec![
            "-9223372036854775809",
            "-9223372036854775808",
            "-2147483648",
            "-32768",
            "-128",
            "-1",
            "0",
            "1",
            "127",
            "32767",
            "2147483647",
            "9223372036854775807",
            "9223372036854775808",
        ];
        let _integers: Vec<Int> = integer_strings
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    #[test]
    fn test_invalid_integers() -> Result<()> {
        let integer_strings = vec!["", "abc", "1.", "1.0", " 10", " -10", "- 10", "10%"];
        let results: Vec<Result<Int>> = integer_strings
            .into_iter()
            .map(|item| item.try_into())
            .collect();

        for result in results {
            assert!(result.is_err())
        }

        Ok(())
    }
}
