/// # Password Hash
///
/// Meant to avoid leaking senstive password data
///
#[derive(derive_more::Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct PasswordHash {
    #[debug(skip)]
    data: String,
}

mod impls {
    use super::*;

    impl<T: Into<String>> From<T> for PasswordHash {
        fn from(value: T) -> Self {
            Self { data: value.into() }
        }
    }

    impl PasswordHash {
        /// To avoid accidentally leaking, if you want
        /// to get the string value you must call
        /// [PasswordHash::as_str]
        ///
        pub fn as_str(&self) -> &str {
            self.data.as_str()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PasswordHash;

    #[test]
    fn deserialize_directly_from_string() {
        let password: PasswordHash = serde_json::from_str("\"the-hash\"").unwrap();
        assert_eq!("the-hash", password.as_str());
    }
}
