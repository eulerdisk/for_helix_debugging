use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl std::str::FromStr for SubscriberName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err("empty".into());
        }
        if s.graphemes(true).count() > 256 {
            return Err("too long".into());
        }

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        if s.chars().any(|g| forbidden_characters.contains(&g)) {
            return Err("forbidden chars".into());
        }

        Ok(SubscriberName(s.into()))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let sub_name: Result<SubscriberName, _> = "a".repeat(256).parse();
        assert_ok!(sub_name);
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let sub_name: Result<SubscriberName, _> = "a".repeat(257).parse();
        assert_err!(sub_name);
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let sub_name: Result<SubscriberName, _> = " ".parse();
        assert_err!(sub_name);
    }

    #[test]
    fn empty_string_is_rejected() {
        let sub_name: Result<SubscriberName, _> = "".parse();
        assert_err!(sub_name);
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let sub_name: Result<SubscriberName, _> = name.to_string().parse();
            assert_err!(sub_name);
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let sub_name: Result<SubscriberName, _> = "Ursula Le Guin".parse();
        assert_ok!(sub_name);
    }
}
