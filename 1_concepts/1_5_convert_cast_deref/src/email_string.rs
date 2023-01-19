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

impl TryFrom<String> for EmailString {
    type Error = &'static str;
    fn try_from(str: String) -> Result<Self, Self::Error> {
        match EmailAddress::is_valid(&str) {
            true => Ok(EmailString(str.to_string())),
            false => Err("invalid string"),
        }
    }
}
impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_try_from_str_err() {
        let b = EmailString::try_from("AAA");
        assert_eq!(b.err().unwrap(), "invalid email");
    }
    #[test]
    fn test_try_from_str_true() {
        let b = EmailString::try_from("tim@apple.com");
        let l = b.ok().unwrap();
        let r = EmailString("tim@apple.com".to_string());
        assert!(l == r);
    }

    #[test]
    fn test_try_from_string_err() {
        let b = EmailString::try_from("AAA".to_string());
        assert_eq!(b.err().unwrap(), "invalid string");
    }
    #[test]
    fn test_try_from_string_true() {
        let b = EmailString::try_from("tim@apple.com".to_string());
        let l = b.ok().unwrap();
        let r = EmailString("tim@apple.com".to_string());
        assert!(l == r);
    }
}
