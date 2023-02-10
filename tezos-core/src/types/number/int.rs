use lazy_static::lazy_static;
use ibig::{IBig, UBig};
use num_traits::ToPrimitive;
use num_integer::Integer;
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, de};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use derive_more::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
    DivAssign, Mul, MulAssign, Octal, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign,
    Sub, SubAssign,
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
#[derive(
    Add,
    AddAssign,
    PartialEq,
    PartialOrd,
    Debug,
    Eq,
    Clone,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Octal,
    Rem,
    RemAssign,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
    Sub,
    SubAssign,
)]
#[div(forward)]
#[div_assign(forward)]
#[mul(forward)]
#[mul_assign(forward)]
#[rem(forward)]
#[rem_assign(forward)]
pub struct Int(IBig);

#[cfg(feature = "serde")]
impl Serialize for Int {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.collect_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Int {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        Self::from(String::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

impl Int {
    pub fn from<S: Into<String>>(value: S) -> Result<Self> {
        let value: String = value.into();
        if Self::is_valid(&value) {
            return Ok(Self(IBig::from_str_radix(&value, 10)?));
        }
        Err(Error::InvalidIntegerString)
    }

    pub fn from_integer<I: Integer + ToString>(value: I) -> Self {
        Self::from_string(value.to_string()).unwrap()
    }

    pub fn to_integer<I: Integer + FromStr>(&self) -> Result<I>
    where
        <I as FromStr>::Err: Debug,
    {
        I::from_str(&self.0.to_string())
            .map_err(|_| Error::InvalidIntegerConversion)
    }

    pub fn from_string(value: String) -> Result<Self> {
        Self::from(value)
    }

    pub fn is_valid(value: &str) -> bool {
        REGEX.is_match(value)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        IntegerBytesCoder::encode(self)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for Int {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_string(s.into())
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToPrimitive for Int {
    fn to_i64(&self) -> Option<i64> {
        TryInto::<i64>::try_into(self.0.clone()).ok()
    }

    fn to_u64(&self) -> Option<u64> {
        TryInto::<u64>::try_into(self.0.clone()).ok()
    }

    fn to_i128(&self) -> Option<i128> {
        TryInto::<i128>::try_into(self.0.clone()).ok()
    }

    fn to_u128(&self) -> Option<u128> {
        TryInto::<u128>::try_into(self.0.clone()).ok()
    }
}

impl From<i8> for Int {
    fn from(value: i8) -> Self {
        Self(IBig::from(value))
    }
}

impl From<i16> for Int {
    fn from(value: i16) -> Self {
        Self(IBig::from(value))
    }
}

impl From<i32> for Int {
    fn from(value: i32) -> Self {
        Self(IBig::from(value))
    }
}

impl From<i64> for Int {
    fn from(value: i64) -> Self {
        Self(IBig::from(value))
    }
}

impl From<i128> for Int {
    fn from(value: i128) -> Self {
        Self(IBig::from(value))
    }
}

impl From<IBig> for Int {
    fn from(value: IBig) -> Self {
        Self(value)
    }
}

impl From<Nat> for Int {
    fn from(value: Nat) -> Self {
        let value: IBig = value.value().into();
        Int(value)
    }
}

impl From<&Nat> for Int {
    fn from(value: &Nat) -> Self {
        let value: IBig = value.value().into();
        Int(value)
    }
}

impl From<&Int> for IBig {
    fn from(value: &Int) -> Self {
        value.0.clone()
    }
}

impl From<Int> for IBig {
    fn from(value: Int) -> Self {
        value.0
    }
}

impl From<Int> for String {
    fn from(value: Int) -> Self {
        value.to_string()
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

impl TryFrom<&Int> for u32 {
    type Error = Error;

    fn try_from(value: &Int) -> Result<u32> {
        Ok(value.0.clone().try_into()?)
    }
}

impl TryInto<i64> for Int {
    type Error = Error;

    fn try_into(self) -> Result<i64> {
        Ok(self.0.try_into()?)
    }
}

impl TryInto<UBig> for Int {
    type Error = Error;

    fn try_into(self) -> Result<UBig> {
        Ok(self.0.try_into()?)
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
