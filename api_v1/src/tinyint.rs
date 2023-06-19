use std::fmt::{Display, Formatter};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TinyInt(pub bool);

impl TryFrom<u8> for TinyInt {
    type Error = TinyIntError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => TinyInt(false),
            1 => TinyInt(true),
            _ => Err(TinyIntError::InvalidValue(value))?,
        })
    }
}

impl Display for TinyInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<bool> for TinyInt {
    fn from(b: bool) -> Self {
        TinyInt(b)
    }
}

impl From<TinyInt> for u8 {
    fn from(ti: TinyInt) -> u8 {
        if ti.0 {
            1
        } else {
            0
        }
    }
}

//
// SERDE
//

impl Serialize for TinyInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(if self.0 { 1u8 } else { 0 })
    }
}

impl<'de> Deserialize<'de> for TinyInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TinyIntVisitor;

        impl<'de> serde::de::Visitor<'de> for TinyIntVisitor {
            type Value = TinyInt;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("invalid value for TinyInt")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let byte: u8 = v.try_into().map_err(|e| E::custom(e))?;
                TinyInt::try_from(byte).map_err(|e| E::custom(e))
            }
        }

        deserializer.deserialize_any(TinyIntVisitor)
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum TinyIntError {
    #[error("Invalid value for TinyInt: \"{0}\"")]
    InvalidValue(u8),
}
