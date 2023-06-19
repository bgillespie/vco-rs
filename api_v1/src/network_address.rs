use mac_address::MacAddress;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::net::{Ipv4Addr, Ipv6Addr};
use thiserror::Error;

pub const UNSET: &str = "unset";

pub trait NetAddress
where
    Self: Sized + Display + ToString,
{
    fn de(value: &str) -> Result<Address<Self>, AddressError>;
    fn ser(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Address<T>
where
    T: NetAddress,
{
    Undefined,
    Unknown,
    Some(T),
}

impl NetAddress for Ipv4Addr {
    fn de(value: &str) -> Result<Address<Self>, AddressError> {
        Ok(match value {
            "" => Address::Undefined,
            _ => Address::Some(
                value
                    .parse::<Ipv4Addr>()
                    .map_err(|e| AddressError::InvalidIpV4Addr(e.to_string()))?,
            ),
        })
    }

    fn ser(&self) -> String {
        self.to_string()
    }
}

impl NetAddress for Ipv6Addr {
    fn de(value: &str) -> Result<Address<Self>, AddressError> {
        Ok(match value {
            "" => Address::Undefined,
            _ => Address::Some(
                value
                    .parse::<Ipv6Addr>()
                    .map_err(|e| AddressError::InvalidIpV6Addr(e.to_string()))?,
            ),
        })
    }

    fn ser(&self) -> String {
        self.to_string()
    }
}

impl NetAddress for MacAddress {
    fn de(value: &str) -> Result<Address<Self>, AddressError> {
        Ok(match value {
            "" => Address::Undefined,
            "UNKNOWN" => Address::Unknown,
            _ => Address::Some(
                value
                    .parse::<MacAddress>()
                    .map_err(|e| AddressError::InvalidMacAddress(e.to_string()))?,
            ),
        })
    }

    fn ser(&self) -> String {
        self.to_string()
    }
}

impl<T> TryFrom<&str> for Address<T>
where
    T: NetAddress,
{
    type Error = AddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(T::de(value).map_err(|_| AddressError::InvalidIpV4Addr(value.into()))?)
    }
}

impl<T> From<&Address<T>> for String
where
    T: NetAddress,
{
    fn from(value: &Address<T>) -> Self {
        match value {
            Address::Undefined => "".to_string(),
            Address::Unknown => "UNKNOWN".to_string(),
            Address::Some(addr) => addr.ser(),
        }
    }
}

impl<T> Display for Address<T>
where
    T: NetAddress,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Address::Undefined => UNSET.into(),
                Address::Unknown => "UNKNOWN".into(),
                Address::Some(v) => format!("{}", v),
            }
        )
    }
}

//
// SERDE
//

impl<T> Serialize for Address<T>
where
    T: NetAddress,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&String::from(self))
    }
}

impl<'de, T> Deserialize<'de> for Address<T>
where
    T: NetAddress,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AddressVisitor<T>
        where
            T: NetAddress,
        {
            _phantom: PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for AddressVisitor<T>
        where
            T: NetAddress,
        {
            type Value = Address<T> where T: NetAddress;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("invalid value for Address")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(T::de(v).map_err(E::custom)?)
            }
        }

        deserializer.deserialize_str(AddressVisitor {
            _phantom: PhantomData,
        })
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AddressError {
    #[error("Invalid value for IpV4: \"{0}\"")]
    InvalidIpV4Addr(String),

    #[error("Invalid value for IpV6: \"{0}\"")]
    InvalidIpV6Addr(String),

    #[error("Invalid value for MAC address: \"{0}\"")]
    InvalidMacAddress(String),
}
