use email_address::EmailAddress;
use std::ops::Deref;
pub struct EmailString(String);
impl PartialEq for EmailString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_try_from_err() {
        let b = EmailString::try_from("AAA");
        assert_eq!(b.err().unwrap(), "invalid email");
    }
    #[test]
    fn test_try_from_true() {
        let b = EmailString::try_from("tim@apple.com");
        let l = b.ok().unwrap();
        let r = EmailString("tim@apple.com".to_string());
        assert!(l == r);
    }
}
