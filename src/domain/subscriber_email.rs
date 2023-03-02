use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct SubscriberEmail(String);

impl FromStr for SubscriberEmail {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if validator::validate_email(s) {
            Ok(Self(s.into()))
        } else {
            Err(format!("{s} is not a valid subscriber email."))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claim::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};

    use super::*;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        let email: Result<SubscriberEmail, _> = valid_email.0.parse();
        email.is_ok()
    }

    #[test]
    fn empty_string_is_rejected() {
        let email: Result<SubscriberEmail, _> = "".parse();
        assert_err!(email);
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email: Result<SubscriberEmail, _> = "ursuladomain.com".parse();
        assert_err!(email);
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email: Result<SubscriberEmail, _> = "@domain.com".parse();
        assert_err!(email);
    }
}
