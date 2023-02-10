use lazy_static::lazy_static;
use ibig::{UBig, IBig};
use num_traits::{ToPrimitive};
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
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder, NaturalBytesCoder},
        consumable_list::ConsumableList,
    },
    types::mutez::Mutez,
    Error, Result,
};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^[0-9]+$").unwrap();
}

/// An unsigned integer that can be encoded to a Zarith number
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
pub struct Nat(UBig);

#[cfg(feature = "serde")]
impl Serialize for Nat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.collect_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Nat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        Self::from(String::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

impl Nat {
    pub fn from<S: Into<String>>(value: S) -> Result<Self> {
        let value: String = value.into();
        if Self::is_valid(&value) {
            return Ok(Self(UBig::from_str_radix(&value, 10)?));
        }
        Err(Error::InvalidIntegerString)
    }

    pub fn from_string(value: String) -> Result<Self> {
        Self::from(value)
    }

    pub(super) fn value(&self) -> &UBig {
        &self.0
    }

    pub fn is_valid(value: &str) -> bool {
        REGEX.is_match(value)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        NaturalBytesCoder::encode(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        NaturalBytesCoder::decode(bytes)
    }

    pub fn from_consumable_bytes<CL: ConsumableList<u8>>(bytes: &mut CL) -> Result<Self> {
        NaturalBytesCoder::decode_consuming(bytes)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl ToPrimitive for Nat {
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

impl FromStr for Nat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_string(s.into())
    }
}

impl Display for Nat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u8> for Nat {
    fn from(value: u8) -> Self {
        Self(UBig::from(value))
    }
}

impl From<u16> for Nat {
    fn from(value: u16) -> Self {
        Self(UBig::from(value))
    }
}

impl From<u32> for Nat {
    fn from(value: u32) -> Self {
        Self(UBig::from(value))
    }
}

impl From<u64> for Nat {
    fn from(value: u64) -> Self {
        Self(UBig::from(value))
    }
}

impl From<u128> for Nat {
    fn from(value: u128) -> Self {
        Self(UBig::from(value))
    }
}

impl From<usize> for Nat {
    fn from(value: usize) -> Self {
        Self(UBig::from(value))
    }
}

impl From<UBig> for Nat {
    fn from(value: UBig) -> Self {
        Self(value)
    }
}

impl From<&Mutez> for Nat {
    fn from(mutez: &Mutez) -> Self {
        Self(UBig::from(mutez.value()))
    }
}

impl From<Nat> for String {
    fn from(value: Nat) -> Self {
        value.0.to_string()
    }
}

impl From<Nat> for UBig {
    fn from(value: Nat) -> Self {
        value.0
    }
}

impl From<Nat> for IBig {
    fn from(value: Nat) -> Self {
        value.0.into()
    }
}

impl TryFrom<Nat> for Mutez {
    type Error = Error;

    fn try_from(value: Nat) -> Result<Self> {
        value.0.try_into()
    }
}

impl TryFrom<String> for Nat {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::from_string(value)
    }
}

impl TryFrom<&str> for Nat {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_string(value.to_string())
    }
}

impl TryFrom<&Vec<u8>> for Nat {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        NaturalBytesCoder::decode(value)
    }
}

impl TryFrom<&Nat> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Nat) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_naturals() -> Result<()> {
        let values = vec![
            "0",
            "1",
            "127",
            "32767",
            "2147483647",
            "9223372036854775807",
            "9223372036854775808",
        ];
        let _result: Vec<Nat> = values
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    #[test]
    fn test_invalid_naturals() -> Result<()> {
        let values = vec![
            "",
            "abc",
            "1.",
            "1.0",
            " 10",
            " -10",
            "- 10",
            "10%",
            "-9223372036854775809",
            "-9223372036854775808",
            "-2147483648",
            "-32768",
            "-128",
            "-1",
        ];
        let results: Vec<Result<Nat>> = values.into_iter().map(|item| item.try_into()).collect();

        for result in results {
            assert!(result.is_err())
        }

        Ok(())
    }
}
