/// # Secret-Secret, I've got a Secret
///
/// Meant to avoid leaking senstive password info
/// as well as provide a common set of validations.
///
#[derive(derive_more::Debug, Clone, validator::Validate)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Password {
    #[debug(skip)]
    #[validate(length(min = 12))]
    data: String,
}

mod impls {
    use super::*;

    impl<T: Into<String>> From<T> for Password {
        fn from(value: T) -> Self {
            Self { data: value.into() }
        }
    }

    impl Password {
        /// To avoid accidentally leaking, if you want
        /// to get the string value you must call
        /// [Password::as_str]
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
    use super::Password;
    use validator::Validate;

    #[test]
    fn validate_success() {
        let password = Password::from("TheP455w0rdIs$tr0ng!");
        password.validate().expect("to validate");
    }

    #[test]
    fn validate_failure() {
        let password = Password::from("password");
        password.validate().expect_err("to not validate");
    }

    #[test]
    fn deserialize_directly_from_string() {
        let password: Password = serde_json::from_str("\"TheP455w0rdIs$tr0ng!\"").unwrap();
        password.validate().expect("to validate");
        assert_eq!("TheP455w0rdIs$tr0ng!", password.as_str());
    }
}
