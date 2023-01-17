use std::ops::Deref;
use email_address::EmailAddress;
pub struct EmailString(String);

impl TryFrom<&str> for EmailString {
    type Error = &'static str;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match EmailAddress::is_valid(str) {
            true => Ok(EmailString(str.to_string())),
            false => Err("invalid email"),
        }
    }
}

impl From<String> for EmailString {
    fn from(str: String) -> Self {
        match EmailAddress::is_valid(&str) {
            true => EmailString(str),
            false => EmailString("".to_string()),
        }
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for EmailString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}