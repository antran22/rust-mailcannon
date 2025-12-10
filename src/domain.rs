use snafu::{Snafu, ensure};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

#[derive(Debug, Snafu)]
#[snafu(display("{input} is not a valid username: {message}"))]
pub struct SubscriberNameError {
    input: String,
    message: String,
}

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, SubscriberNameError> {
        ensure!(
            !s.trim().is_empty(),
            SubscriberNameSnafu {
                input: s,
                message: "it's empty"
            }
        );

        ensure!(
            s.graphemes(true).count() <= 256,
            SubscriberNameSnafu {
                input: s,
                message: "it's too long"
            }
        );

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        ensure!(
            !contains_forbidden_characters,
            SubscriberNameSnafu {
                input: s,
                message: "it contains invalid character"
            }
        );

        Ok(SubscriberName(s))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ë".repeat(256);
        let name = SubscriberName::parse(name);
        assert!(
            name.is_ok(),
            "should be able to parse a 256-grapheme long string"
        )
    }

    #[test]
    fn a_257_grapheme_long_name_is_invalid() {
        let name = "ë".repeat(257);
        let name = SubscriberName::parse(name);
        assert!(name.is_err(), "should fail on a 257-grapheme long string")
    }

    #[test]
    fn empty_string_is_invalid() {
        let name = SubscriberName::parse("".to_string());

        assert!(name.is_err(), "should fail on empty string")
    }

    #[test]
    fn invalid_character_is_invalid() {
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        for chr in &forbidden_characters {
            let name = SubscriberName::parse(chr.to_string());
            assert!(name.is_err(), "should fail on invalid character")
        }
    }

    #[test]
    fn normal_name_should_pass() {
        let name = SubscriberName::parse("Phillip K. Dick".to_string());
        assert!(name.is_ok(), "a normal name should pass")
    }
}
