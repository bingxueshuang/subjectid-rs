use std::fmt::{Display, Formatter};
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize};

use crate::Error;

/// [PhoneNumber] defines the [`E.164`] formatted telephone numbers. The number may optionally
/// begin with '+'. Maximum 15 digit telephone number is comprised of a CC (country code) prefix
/// part and an identifier part.
///
/// [`E.164`]: https://www.itu.int/rec/T-REC-E.164-201011-I/en
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct PhoneNumber {
    number: String,
}

/// Regular expression that defines E.164 telephone number structure for parsing.
static RE_PHONE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\+?(\d{1,15})$").unwrap());

/// Phone number parsing rules
impl PhoneNumber {
    fn parse(s: &str) -> Result<Self, Error> {
        let Some(caps) = RE_PHONE.captures(s) else {
            return Err(Error::InvalidPhoneNumber);
        };
        let number = "+".to_owned() + &caps[1];
        Ok(Self { number })
    }
}

impl FromStr for PhoneNumber {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl<'de> Deserialize<'de> for PhoneNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(PhVisitor)
    }
}

struct PhVisitor;

impl<'de> Visitor<'de> for PhVisitor {
    type Value = PhoneNumber;
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an E.164 formatted phone number")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Self::Value::parse(v).map_err(|e| de::Error::custom(e))
    }
}
