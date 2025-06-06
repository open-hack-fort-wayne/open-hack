/// # Email Address
///
/// Meant to avoid leaking senstive email addresses
/// as well as provide a common set of validation.
///
#[derive(derive_more::Debug, Clone, validator::Validate, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Email {
    #[debug(skip)]
    #[validate(email)]
    data: String,
}

mod impls {
    use super::*;

    impl<T: Into<String>> From<T> for Email {
        fn from(value: T) -> Self {
            Self { data: value.into() }
        }
    }

    impl Email {
        /// To avoid accidentally leaking, if you want
        /// to get the string value you must call
        /// [Email::as_str]
        ///
        pub fn as_str(&self) -> &str {
            self.data.as_str()
        }

        /// Unwraps outer type and returns the [String]
        /// data holding the email value.
        pub fn into_inner(self) -> String {
            self.data
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use validator::Validate;

    #[test]
    fn validate_success() {
        let email = Email::from("bfalk@rofl.com");
        email.validate().expect("to validate");
    }

    #[test]
    fn validate_failure() {
        let email = Email::from("rdog");
        email.validate().expect_err("to not validate");
    }

    #[test]
    fn deserialize_directly_from_string() {
        let email: Email = serde_json::from_str("\"bfalk@rofl.com\"").unwrap();
        email.validate().expect("to validate");
        assert_eq!("bfalk@rofl.com", email.as_str());
    }
}
